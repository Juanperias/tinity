use super::regs::Reg;

#[derive(Debug)]
pub struct RegisterInstruction {
    funct3: u64,
    funct7: u64,
    rs2: u64,
    rs1: u64,
    rd: u64,
    opcode: u64,
}

fn register_to_endian(ins: RegisterInstruction) -> Vec<u8> {
    let instruction: u32 = ((ins.funct7 as u32) << 25)
        | ((ins.rs2 as u32) << 20)
        | ((ins.rs1 as u32) << 15)
        | ((ins.funct3 as u32) << 12)
        | ((ins.rd as u32) << 7)
        | (ins.opcode as u32);

    instruction.to_le_bytes().to_vec()
}

pub fn add(rd: &Reg, rs1: &Reg, rs2: &Reg) -> Vec<u8> {
    register_to_endian(RegisterInstruction {
        funct3: 0x0,
        funct7: 0x0,
        rs1: rs1.into(),
        rs2: rs2.into(),
        rd: rd.into(),
        opcode: 0b0110011,
    })
}

pub fn sub(rd: &Reg, rs1: &Reg, rs2: &Reg) -> Vec<u8> {
    register_to_endian(RegisterInstruction {
        funct3: 0x0,
        funct7: 0x20,
        rs1: rs1.into(),
        rs2: rs2.into(),
        rd: rd.into(),
        opcode: 0b110011,
    })
}
