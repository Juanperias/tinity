use super::immediate::{addi, ecall};
use super::jmp::{jal, jarl};
use super::regs::Reg;
use crate::parser::ast::AstNode;
use std::convert::TryFrom;
use std::collections::HashMap;

type Opcode = Vec<u8>;

pub fn node_to_opcode(node: AstNode, functions: &HashMap<String, u64>) -> Opcode {
    let mut opcode = Vec::new();
    match node {
        AstNode::Function { .. } => {}
        AstNode::Sum { .. } => {}
        AstNode::Load { dist, value } => {
            if let Ok(reg) = Reg::try_from(dist.clone()) {
                opcode.extend(addi(reg, Reg::Zero, value));
            } else {
                eprintln!("Invalid register: {}", dist);
            }
        }
        AstNode::Syscall => {
            opcode.extend(ecall());
        },
        AstNode::Go { target, pc } => {
            //TODO: improve Option handling instead of using unwrap
            opcode.extend(jal(*functions.get(&target).unwrap(), pc, Reg::Ra));
        },
        AstNode::Ret => {
            opcode.extend(jarl(Reg::Zero, Reg::Ra, 0));
        }
    }
    opcode
}

pub fn from_nodes(nodes: Vec<AstNode>, functions: &HashMap<String, u64>) -> Opcode {
    nodes.into_iter().flat_map(|node| node_to_opcode(node, functions)).collect()
}
