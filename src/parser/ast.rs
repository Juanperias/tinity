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

    for token in tokens {
        match token {
            Token::Fn(name) => {
                if current_function.is_some() {
                    return Err(AstError::NestedFunction);
                }

                current_function = Some(CurrentFunction {
                    name,
                    body: Vec::new(),
                    pc: current_pc,
                });
            }
            Token::Radd(params) => {
                let current = match current_function.as_mut() {
                    Some(s) => s,
                    None => return Err(AstError::OutsideOfFunction),
                };

                current.body.push(AstNode::Radd {
                    target: params.0,
                    rs1: params.1,
                });

                current_pc += 4;
            }
            Token::Rsub(params) => {
                let current = match current_function.as_mut() {
                    Some(s) => s,
                    None => return Err(AstError::OutsideOfFunction),
                };

                current.body.push(AstNode::Rsub {
                    target: params.0,
                    rs1: params.1,
                });

                current_pc += 4;
            }
            Token::Sum(params) => {
                let current = match current_function.as_mut() {
                    Some(s) => s,
                    None => return Err(AstError::OutsideOfFunction),
                };

                let count = params
                    .1
                    .iter()
                    .filter(|&x| matches!(x, Type::Value(value) if value != "%zero"))
                    .count();

                current.body.push(AstNode::Sum {
                    dist: params.0,
                    numbers: params.1,
                    t: params.2,
                });

                current_pc += 4 + count as u64;
            }
            Token::Nop => {
                let current = match current_function.as_mut() {
                    Some(s) => s,
                    None => return Err(AstError::OutsideOfFunction),
                };

                current.body.push(AstNode::Nop);

                current_pc += 4;
            }
            Token::Go(target) => {
                let current = match current_function.as_mut() {
                    Some(s) => s,
                    None => return Err(AstError::OutsideOfFunction),
                };
                current.body.push(AstNode::Go {
                    pc: current_pc,
                    target,
                });

                current_pc += 4;
            }
            Token::Syscall => {
                let current = match current_function.as_mut() {
                    Some(s) => s,
                    None => return Err(AstError::OutsideOfFunction),
                };
                current.body.push(AstNode::Syscall);

                current_pc += 4;
            }
            Token::Load(params) => {
                let current = match current_function.as_mut() {
                    Some(s) => s,
                    None => return Err(AstError::OutsideOfFunction),
                };

                current.body.push(AstNode::Load {
                    dist: params.0,
                    value: params.1,
                });

                current_pc += 4;
            }
            Token::Global => {
                stype = SymbolType::Global;
            }
            Token::Ret => {
                let current = match current_function.as_mut() {
                    Some(s) => s,
                    None => return Err(AstError::OutsideOfFunction),
                };

                current.body.push(AstNode::Ret);

                current_pc += 4;
            }
            Token::EndFn => {
                let current = match current_function.take() {
                    Some(s) => s,
                    None => return Err(AstError::EndFnWithoutFn),
                };
                functions.push(AstNode::Function {
                    name: current.name.clone(),
                    body: current.body,
                    pc: current.pc,
                    stype,
                });

                functions_hashmap.insert(current.name, current.pc);
            }
        }
    }

    if current_function.is_some() {
        return Err(AstError::FnNotClosed);
    }

    Ok((functions, functions_hashmap))
}
