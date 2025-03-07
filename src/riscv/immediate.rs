use super::regs::Reg;

#[derive(Debug)]
pub struct ImmediateInstruction {
    pub opcode: u32,
    pub rd: Reg,
    pub rs1: Reg,
    pub imm: i64,
}

pub fn immediate_to_endian(ins: ImmediateInstruction) -> Vec<u8> {
    let rd: u64 = ins.rd.into();
    let rs1: u64 = ins.rs1.into();
    let instruction = (ins.imm as u32) << 20
        | (rs1 as u32) << 15
        | (0x00 as u32) << 12
        | (rd as u32) << 7
        | ins.opcode as u32;
    instruction.to_le_bytes().to_vec()
}

pub fn ecall() -> Vec<u8> {
    immediate_to_endian(ImmediateInstruction {
        opcode: 0x73,
        rd: Reg::Zero,
        rs1: Reg::Zero,
        imm: 0,
    })
}

pub fn addi(dist: Reg, rs1: Reg, val: i64) -> Vec<u8> {
    immediate_to_endian(ImmediateInstruction {
        opcode: 0x13,
        rs1,
        rd: dist,
        imm: val
    })
}
