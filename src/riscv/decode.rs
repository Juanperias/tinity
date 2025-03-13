use super::immediate::{addi, ecall};
use super::jmp::{jal, jalr, JmpError};
use super::regs::Reg;
use crate::parser::ast::AstNode;
use std::collections::HashMap;
use std::convert::TryFrom;
use crate::parser::types::{Type, TypeError};
use thiserror::Error;
use crate::type_from_string;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Invalid Register {0}")]
    InvalidRegister(String),

    #[error("Target function {0}, not found")]
    FnNotFound(String),

    #[error("{0}")]
    JmpError(#[from] JmpError),

    #[error("{0}")]
    TypeError(#[from] TypeError)
}

type Opcode = Vec<u8>;

pub fn node_to_opcode(
    node: AstNode,
    functions: &HashMap<String, u64>,
) -> Result<Opcode, DecodeError> {
    let mut opcode = Vec::new();
    match node {
        AstNode::Function { .. } => {}
        AstNode::Sum { numbers, dist, t } => {
            let reg = match Reg::try_from(dist.clone()) {
                Ok(r) => r,
                Err(_) => return Err(DecodeError::InvalidRegister(dist)),
            };
            let mut result = type_from_string!(t.as_str(), "0");

            for n in numbers {
                match n {
                    Type::Value(v) => {
                        match v.as_str() {
                            "%zero" => {},
                            _ => {}
                        }
                    },
                    num => { result.try_add(num)? }
                }
            }

            opcode.extend(addi(reg, Reg::Zero, result.try_into()?));
        }
        AstNode::Load { dist, value } => {
            let reg = match Reg::try_from(dist.clone()) {
                Ok(r) => r,
                Err(_) => return Err(DecodeError::InvalidRegister(dist)),
            };
            opcode.extend(addi(reg, Reg::Zero, value));
        }
        AstNode::Syscall => {
            opcode.extend(ecall());
        }
        AstNode::Go { target, pc } => {
            let target_address = match functions.get(&target) {
                Some(s) => s,
                None => return Err(DecodeError::FnNotFound(target)),
            };
            opcode.extend(jal(*target_address, pc, Reg::Ra)?);
        }
        AstNode::Ret => {
            opcode.extend(jalr(Reg::Zero, Reg::Ra, 0));
        }
        AstNode::Nop => {
            opcode.extend(addi(Reg::Zero, Reg::Zero, 0));
        }
    }
    Ok(opcode)
}

pub fn from_nodes(
    nodes: Vec<AstNode>,
    functions: &HashMap<String, u64>,
) -> Result<Opcode, DecodeError> {
    let mut combined_opcode = Vec::new();

    for node in nodes {
        let node_opcode = node_to_opcode(node, functions)?;
        combined_opcode.extend(node_opcode);
    }

    Ok(combined_opcode)
}
