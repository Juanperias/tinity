use super::token::Token;
use super::types::Type;
use crate::binary::symbol::SymbolType;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AstError {
    #[error("Nested functions are not allowed")]
    NestedFunction,

    #[error("Code outside of Function")]
    OutsideOfFunction,

    #[error("A function has not been closed")]
    FnNotClosed,

    #[error("EndFn without matching Fn")]
    EndFnWithoutFn,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstNode {
    Function {
        name: String,
        stype: SymbolType,
        body: Vec<AstNode>,
        pc: u64,
    },
    Sum {
        numbers: Vec<Type>,
        dist: String,
        t: String,
    },
    Load {
        dist: String,
        value: i64,
    },
    Syscall,
    Go {
        target: String,
        pc: u64,
    },
    Radd {
        target: String,
        rs1: String,
    },
    Rsub {
        target: String,
        rs1: String,
    },
    Ret,
    Nop,
}

pub fn get_from_tokens(
    tokens: Vec<Token>,
) -> Result<(Vec<AstNode>, HashMap<String, u64>), AstError> {
    #[derive(Debug)]
    struct CurrentFunction {
        name: String,
        body: Vec<AstNode>,
        pc: u64,
    }

    let mut stype = SymbolType::Private;
    let mut functions = Vec::new();
    let mut current_function: Option<CurrentFunction> = None;
    let mut current_pc = 0x0;
    // Contains the PC addr of all functions
    let mut functions_hashmap = HashMap::new();

    if current_function.is_some() {
        return Err(AstError::FnNotClosed);
    }

    Ok((functions, functions_hashmap))
}
