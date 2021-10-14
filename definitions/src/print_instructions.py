insts = [
    "VSETVLI",
    "VSETIVLI",
    "VSETVL",
    "VLE8_V",
    "VLE16_V",
    "VLE32_V",
    "VLE64_V",
    "VLE128_V",
    "VLE256_V",
    "VLE512_V",
    "VLE1024_V",
    "VSE8_V",
    "VSE16_V",
    "VSE32_V",
    "VSE64_V",
    "VSE128_V",
    "VSE256_V",
    "VSE512_V",
    "VSE1024_V",
    "VADD_VV",
    "VADD_VX",
    "VADD_VI",
    "VSUB_VV",
    "VSUB_VX",
    "VRSUB_VX",
    "VRSUB_VI",
    "VMUL_VV",
    "VMUL_VX",
    "VDIV_VV",
    "VDIV_VX",
    "VDIVU_VV",
    "VDIVU_VX",
    "VREM_VV",
    "VREM_VX",
    "VREMU_VV",
    "VREMU_VX",
    "VSLL_VV",
    "VSLL_VX",
    "VSLL_VI",
    "VSRL_VV",
    "VSRL_VX",
    "VSRL_VI",
    "VSRA_VV",
    "VSRA_VX",
    "VSRA_VI",
    "VMSEQ_VV",
    "VMSEQ_VX",
    "VMSEQ_VI",
    "VMSNE_VV",
    "VMSNE_VX",
    "VMSNE_VI",
    "VMSLTU_VV",
    "VMSLTU_VX",
    "VMSLT_VV",
    "VMSLT_VX",
    "VMSLEU_VV",
    "VMSLEU_VX",
    "VMSLEU_VI",
    "VMSLE_VV",
    "VMSLE_VX",
    "VMSLE_VI",
    "VMSGTU_VX",
    "VMSGTU_VI",
    "VMSGT_VX",
    "VMSGT_VI",
]

for (i, e) in enumerate(insts):
    print(f'pub const OP_{e}: InstructionOpcode = 0x{i:02x}f0;')
