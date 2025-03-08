use super::immediate::{addi, ecall};
use super::jmp::{jal, jarl};
use super::regs::Reg;
use crate::parser::ast::AstNode;
use std::convert::TryFrom;
use std::collections::HashMap;
use anyhow::{Result, Context};

type Opcode = Vec<u8>;

pub fn node_to_opcode(node: AstNode, functions: &HashMap<String, u64>) -> Result<Opcode> {
    let mut opcode = Vec::new();
    match node {
        AstNode::Function { .. } => {}
        AstNode::Sum { .. } => {}
        AstNode::Load { dist, value } => {
            let reg = Reg::try_from(dist.clone())
                .map_err(|_| anyhow::anyhow!("Invalid register: {}", dist))?;
            opcode.extend(addi(reg, Reg::Zero, value));
        }
        AstNode::Syscall => {
            opcode.extend(ecall());
        },
        AstNode::Go { target, pc } => {
            let target_address = functions.get(&target)
                .with_context(|| format!("Objective function not found: {}", target))?;
            opcode.extend(jal(*target_address, pc, Reg::Ra));
        },
        AstNode::Ret => {
            opcode.extend(jarl(Reg::Zero, Reg::Ra, 0));
        }
    }
    Ok(opcode)
}

pub fn from_nodes(nodes: Vec<AstNode>, functions: &HashMap<String, u64>) -> Result<Opcode> {
    let mut combined_opcode = Vec::new();

    for node in nodes {
        let node_opcode = node_to_opcode(node, functions)
            .with_context(|| "Error processing AST node")?;
        combined_opcode.extend(node_opcode);
    }

    Ok(combined_opcode)
}
