use super::immediate::{immediate_to_endian, ImmediateInstruction};
use super::regs::Reg;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JmpError {
    #[error("Target is not aligned correctly")]
    NonAlignedAddress,
}

pub fn jalr(dist: Reg, rs1: Reg, offset: i64) -> Vec<u8> {
    immediate_to_endian(ImmediateInstruction {
        rs1,
        rd: dist,
        imm: offset,
        opcode: 0x67,
    })
}

pub fn jal(target_pc: u64, current_pc: u64, rd: Reg) -> Result<Vec<u8>, JmpError> {
    let offset = target_pc.wrapping_sub(current_pc) as i32;

    let rd: u64 = rd.into();

    if offset < -1048576 || offset > 1048574 {
        // This is supposed to be replaced with a long jal
        panic!("invalid offset: too big");
    }

    if (offset % 2) != 0 {
        return Err(JmpError::NonAlignedAddress);
    }

    let offset_in_units = offset / 2;

    let imm = ((offset_in_units & 0x80000) << 12)
        | ((offset_in_units & 0x3FF) << 21)
        | ((offset_in_units & 0x400) << 10)
        | ((offset_in_units & 0x7F800) >> 11);

    let instruction = imm as u32 | (rd as u32) << 7 | 0x6F;

    Ok(instruction.to_le_bytes().to_vec())
}
