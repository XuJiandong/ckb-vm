#![no_main]
use std::convert::TryInto;

use ckb_vm::{
    elf::{LoadingAction, ProgramMetadata},
    machine::VERSION2,
    memory::{round_page_down, round_page_up, FLAG_EXECUTABLE, FLAG_FREEZED},
    snapshot2::{DataSource, Snapshot2Context},
    Bytes, CoreMachine, DefaultMachine, DefaultMachineBuilder, Error, Memory, ISA_A, ISA_B,
    ISA_IMC, ISA_MOP,
};
use ckb_vm_definitions::{asm::AsmCoreMachine, RISCV_MAX_MEMORY};
use libfuzzer_sys::fuzz_target;

const DATA_SOURCE_PROGRAM: u32 = 0;
const DATA_SOURCE_OTHERS: u32 = 1;

#[derive(Clone)]
pub struct DummyData {
    pub program: Bytes,
    pub data: Bytes,
}

pub struct StoreBytesAction {
    pub addr: u64,
    pub offset: u64,
    pub length: u64,
}

pub trait Fuzzing {
    const SIZE: usize = 0;
    fn new_from_data(data: &[u8]) -> Self;
}

impl Fuzzing for StoreBytesAction {
    const SIZE: usize = 12;
    fn new_from_data(data: &[u8]) -> Self {
        if data.len() < Self::SIZE {
            return Self {
                addr: 0,
                offset: 0,
                length: 0,
            };
        }
        let addr =
            (u32::from_le_bytes(data[0..4].try_into().unwrap()) % RISCV_MAX_MEMORY as u32) as u64;
        let offset =
            (u32::from_le_bytes(data[4..8].try_into().unwrap()) % RISCV_MAX_MEMORY as u32) as u64;
        let length =
            (u32::from_le_bytes(data[8..12].try_into().unwrap()) % RISCV_MAX_MEMORY as u32) as u64;
        Self {
            addr,
            offset,
            length,
        }
    }
}

impl Fuzzing for Vec<StoreBytesAction> {
    fn new_from_data(data: &[u8]) -> Self {
        if data.len() < StoreBytesAction::SIZE {
            return vec![];
        }
        data.chunks(StoreBytesAction::SIZE)
            .map(|d| StoreBytesAction::new_from_data(d))
            .collect()
    }
}

impl Fuzzing for DummyData {
    const SIZE: usize = 4;
    fn new_from_data(data: &[u8]) -> Self {
        if data.len() < 4 {
            return Self {
                program: Bytes::new(),
                data: Bytes::new(),
            };
        }
        // TODO: make it more?
        let program_size = u16::from_le_bytes(data[0..2].try_into().unwrap()) as usize;
        let data_size = u16::from_le_bytes(data[2..4].try_into().unwrap()) as usize;
        let program = vec![0xFF as u8; program_size].into();
        let data = vec![0xFF as u8; data_size].into();
        Self { program, data }
    }
}

impl Fuzzing for LoadingAction {
    const SIZE: usize = 16;
    fn new_from_data(data: &[u8]) -> Self {
        if data.len() < Self::SIZE {
            return Self {
                addr: 0,
                size: 0,
                flags: 0,
                source: 0..0,
                offset_from_addr: 0,
            };
        }
        // mimic parse_elf
        let mut p_vaddr = u32::from_le_bytes(data[0..4].try_into().unwrap());
        p_vaddr = p_vaddr % RISCV_MAX_MEMORY as u32;

        let mut p_memsz = u32::from_le_bytes(data[4..8].try_into().unwrap());
        p_memsz = p_memsz % RISCV_MAX_MEMORY as u32;

        let mut p_offset = u32::from_le_bytes(data[8..12].try_into().unwrap());
        p_offset = p_offset % RISCV_MAX_MEMORY as u32;

        let mut p_filesz = u32::from_le_bytes(data[12..16].try_into().unwrap());
        p_filesz = p_filesz % RISCV_MAX_MEMORY as u32;

        let aligned_start = round_page_down(p_vaddr as u64);
        let padding_start = (p_vaddr as u64).wrapping_sub(aligned_start);

        let size = round_page_up((p_memsz as u64).wrapping_add(padding_start));
        let slice_start = p_offset;
        let slice_end = p_offset.wrapping_add(p_filesz);
        if slice_start > slice_end {
            panic!("ElfSegmentAddrOrSizeError");
        }
        Self {
            addr: aligned_start,
            size,
            flags: FLAG_EXECUTABLE | FLAG_FREEZED,
            source: slice_start as u64..slice_end as u64,
            offset_from_addr: padding_start,
        }
    }
}

impl Fuzzing for Vec<LoadingAction> {
    fn new_from_data(data: &[u8]) -> Self {
        if data.len() < LoadingAction::SIZE {
            return vec![];
        }
        data.chunks(LoadingAction::SIZE)
            .map(|d| LoadingAction::new_from_data(d))
            .collect()
    }
}

impl DataSource<u32> for DummyData {
    fn load_data(&self, id: &u32, offset: u64, length: u64) -> Result<(Bytes, u64), Error> {
        let data = match *id {
            DATA_SOURCE_PROGRAM => &self.program,
            DATA_SOURCE_OTHERS => &self.data,
            _ => {
                panic!("invalid id")
            }
        };
        let offset = std::cmp::min(offset as usize, data.len());
        let full_length = data.len() - offset;
        let slice_length = if length > 0 {
            std::cmp::min(full_length, length as usize)
        } else {
            full_length
        };
        Ok((
            data.slice(offset..offset + slice_length),
            full_length as u64,
        ))
    }
}

fn build_machine() -> DefaultMachine<Box<AsmCoreMachine>> {
    let isa = ISA_IMC | ISA_A | ISA_B | ISA_MOP;
    let core_machine = AsmCoreMachine::new(isa, VERSION2, u64::max_value());
    DefaultMachineBuilder::new(core_machine).build()
}

fuzz_target!(|data: &[u8]| {
    let actions_length = LoadingAction::SIZE * 2;
    let dummy_length = DummyData::SIZE;
    let total_length = actions_length + dummy_length;
    if data.len() < total_length {
        return;
    }
    let mut machine1 = build_machine();
    let mut machine2 = build_machine();

    let actions = Vec::<LoadingAction>::new_from_data(&data[0..actions_length]);
    let dummy_data = DummyData::new_from_data(&data[actions_length..actions_length + dummy_length]);
    let mut ctx = Snapshot2Context::new(dummy_data);
    let metadata = ProgramMetadata { actions, entry: 0 };
    let result = ctx.mark_program(&mut machine1, &metadata, &0, 0);
    if result.is_err() {
        return;
    }
    // TODO: store_bytes
    let result = ctx.make_snapshot(&mut machine1);
    if result.is_err() {
        return;
    }
    let snapshot = result.unwrap();
    let result = ctx.resume(&mut machine2, &snapshot);
    if result.is_err() {
        return;
    }
    let mem1 = machine1
        .memory_mut()
        .load_bytes(0, RISCV_MAX_MEMORY as u64)
        .unwrap();
    let mem2 = machine2
        .memory_mut()
        .load_bytes(0, RISCV_MAX_MEMORY as u64)
        .unwrap();
    if mem1 != mem2 {
        panic!("The memory restored by operation resume is not same as snapshot operation");
    }
});
