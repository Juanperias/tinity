use crate::parser::ast::AstNode;
use super::regs::Reg;
use super::immediate::{addi, ecall};
use std::convert::TryFrom;

type Opcode = Vec<u8>;

pub fn node_to_opcode(node: AstNode) -> Opcode {
    let mut opcode = Vec::new();
    match node {
        AstNode::Function { .. } => {},
        AstNode::Sum { .. } => {},
        AstNode::Load { dist, value } => {
            if let Ok(reg) = Reg::try_from(dist.clone()) {
                opcode.extend(addi(reg, Reg::Zero, value));
            } else {
                eprintln!("Invalid register: {}", dist);
            }
        },
        AstNode::Syscall => {
            opcode.extend(ecall());
        }
    }
    opcode
}

pub fn from_nodes(nodes: Vec<AstNode>) -> Opcode {
    nodes.into_iter()
         .flat_map(node_to_opcode)
         .collect()
}

