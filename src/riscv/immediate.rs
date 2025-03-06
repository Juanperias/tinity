#[derive(Debug)]
pub struct ImmediateInstruction {
    pub opcode: u32,
    pub rd: u64,
    pub rs1: u64,
    pub imm: i64,
}

pub fn immediate_to_endian(ins: ImmediateInstruction) -> Vec<u8> {
    let instruction = (ins.imm as u32) << 20
        | (ins.rs1 as u32) << 15
        | (0x00 as u32) << 12
        | (ins.rd as u32) << 7
        | ins.opcode as u32;
    instruction.to_le_bytes().to_vec()
}


