use crate::parser::ast::AstNode;
use super::regs::Reg;
use super::immediate::{addi, ecall};

pub fn node_to_opcode(node: AstNode) -> Vec<u8> {
    let mut opcode = Vec::new();
    match node {
        AstNode::Function { name, body } => {},
        AstNode::Sum { numbers, dist } => {},
        AstNode::Load { dist, value } => {
            opcode.extend(addi(Reg::try_from(dist).unwrap(), Reg::Zero, value));
        },
        AstNode::Syscall => {
            opcode.extend(ecall());
        }
    }
    opcode
}
