#![no_main]
use ckb_vm::{
    elf::{LoadingAction, ProgramMetadata},
    machine::VERSION2,
    memory::{round_page_down, round_page_up, FLAG_EXECUTABLE, FLAG_FREEZED},
    snapshot2::{DataSource, Snapshot2Context},
    Bytes, CoreMachine, DefaultMachine, DefaultMachineBuilder, Error, Memory, ISA_A, ISA_B,
    ISA_IMC, ISA_MOP,
};
use ckb_vm_definitions::{asm::AsmCoreMachine, DEFAULT_MEMORY_SIZE};
use libfuzzer_sys::fuzz_target;
use std::collections::VecDeque;

struct Deque {
    n: VecDeque<u8>,
}

impl Deque {
    fn new(data: [u8; 32]) -> Self {
        Self {
            n: VecDeque::from(data),
        }
    }

    fn u8(&mut self) -> u8 {
        let r = self.n.pop_front().unwrap();
        self.n.push_back(r);
        r
    }

    fn u16(&mut self) -> u16 {
        let mut r = [0u8; 2];
        r.fill_with(|| self.u8());
        u16::from_le_bytes(r)
    }

    fn u32(&mut self) -> u32 {
        let mut r = [0u8; 4];
        r.fill_with(|| self.u8());
        u32::from_le_bytes(r)
    }
}

const DATA_SOURCE_PROGRAM: u32 = 0;
const DATA_SOURCE_CONTENT: u32 = 1;

#[derive(Clone)]
pub struct DummyData {
    pub program: Bytes,
    pub content: Bytes,
}

impl DataSource<u32> for DummyData {
    fn load_data(&self, id: &u32, offset: u64, length: u64) -> Result<(Bytes, u64), Error> {
        let data = match *id {
            DATA_SOURCE_PROGRAM => &self.program,
            DATA_SOURCE_CONTENT => &self.content,
            _ => unreachable!(),
        };
        let offset = std::cmp::min(offset as usize, data.len());
        let full_size = data.len() - offset;
        let real_size = std::cmp::min(full_size, length as usize);
        Ok((data.slice(offset..offset + real_size), full_size as u64))
    }
}

fn build_machine() -> DefaultMachine<Box<AsmCoreMachine>> {
    let isa = ISA_IMC | ISA_A | ISA_B | ISA_MOP;
    let core_machine = AsmCoreMachine::new(isa, VERSION2, u64::max_value());
    DefaultMachineBuilder::new(core_machine).build()
}

fuzz_target!(|data: [u8; 32]| {
    let mut deque = Deque::new(data);
    let dummy_data = {
        let mut program = vec![0u8; deque.u16() as usize + 1024];
        for i in 0..program.len() {
            program[i] = (i % 256) as u8;
        }
        let mut content = vec![0u8; deque.u16() as usize + 1024];
        for i in 0..content.len() {
            content[i] = (i / 256) as u8;
        }
        DummyData {
            program: program.into(),
            content: content.into(),
        }
    };
    let mut loading_action_vec: Vec<LoadingAction> = Vec::new();
    for i in 0..2 + deque.u8() as usize % 3 {
        let segsize = deque.u16() as u64 % 1024;
        let p_vaddr = i as u64 * 1024 * 256;
        let p_memsz = segsize;
        let p_offset = deque.u32() as u64 % (dummy_data.program.len() as u64 - segsize);
        let p_filesz = segsize;
        let aligned_start = round_page_down(p_vaddr);
        let padding_start = (p_vaddr).wrapping_sub(aligned_start);
        let size = round_page_up((p_memsz).wrapping_add(padding_start));
        let slice_start = p_offset;
        let slice_end = p_offset.wrapping_add(p_filesz);
        assert!(slice_start <= slice_end);
        loading_action_vec.push(LoadingAction {
            addr: aligned_start,
            size,
            flags: FLAG_EXECUTABLE | FLAG_FREEZED,
            source: slice_start as u64..slice_end as u64,
            offset_from_addr: padding_start,
        })
    }
    let mut ctx = Snapshot2Context::new(dummy_data.clone());
    let metadata = ProgramMetadata {
        actions: loading_action_vec,
        entry: 0,
    };

    let mut machine1 = build_machine();
    let mut machine2 = build_machine();
    machine1
        .load_program_with_metadata(&dummy_data.program, &metadata, &vec![])
        .unwrap();
    ctx.mark_program(&mut machine1, &metadata, &0, 0).unwrap();
    for i in 0..2 + deque.u8() as usize % 3 {
        let id = i as u32 % 2;
        let data = match id {
            DATA_SOURCE_PROGRAM => &dummy_data.program,
            DATA_SOURCE_CONTENT => &dummy_data.content,
            _ => unreachable!(),
        };
        let length = deque.u16() as u64 % 1024;
        let offset = deque.u32() as u64 % (data.len() as u64 - length);
        let addr = 1024 * 1024 + (deque.u8() as u64 % 4) * 1024 * 256;
        ctx.store_bytes(&mut machine1, addr, &id, offset, length)
            .unwrap();
    }
    let snapshot = ctx.make_snapshot(&mut machine1).unwrap();
    ctx.resume(&mut machine2, &snapshot).unwrap();
    let mem1 = machine1
        .memory_mut()
        .load_bytes(0, DEFAULT_MEMORY_SIZE as u64)
        .unwrap();
    let mem2 = machine2
        .memory_mut()
        .load_bytes(0, DEFAULT_MEMORY_SIZE as u64)
        .unwrap();
    assert_eq!(mem1, mem2);
});
