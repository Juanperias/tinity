use super::token::Token;
use anyhow::{Result, anyhow};
use crate::binary::symbol::SymbolType;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstNode {
    Function {
        name: String,
        stype: SymbolType,
        body: Vec<AstNode>,
        pc: u64,
    },
    Sum {
        numbers: Vec<i64>,
        dist: String
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
    Ret
}

pub fn get_from_tokens(tokens: Vec<Token>) -> Result<(Vec<AstNode>, HashMap<String, u64>)> {
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
                    return Err(anyhow!(format!("Nested function {} is not allowed", name))); 
                }

                current_function = Some(CurrentFunction {
                    name,                    
                    body: Vec::new(),
                    pc: current_pc,
                });
            }
            Token::Sum(params) => {
                let current = current_function.as_mut().expect("Sum outside of function");
                if params.1.len() < 2 {
                    return Err(anyhow!("In a sum there must have been at least two parameters"));
                }

                current.body.push(AstNode::Sum {
                    dist: params.0,
                    numbers: params.1,
                });
                
                current_pc += 4;
            },
            Token::Go(target) => {
                let current = current_function.as_mut().expect("Go outside of function");
                current.body.push(AstNode::Go {
                    pc: current_pc,
                    target
                });

                current_pc += 4;
            },
            Token::Syscall => {
                let current = current_function.as_mut().expect("Syscall outside of function");
                current.body.push(AstNode::Syscall);
                
                current_pc += 4;
            }
            Token::Load(params) => {
                let current = current_function.as_mut().expect("Load outside of function");

                current.body.push(AstNode::Load {
                    dist: params.0,
                    value: params.1
                });

                current_pc += 4;
            },
            Token::Global => {
                stype = SymbolType::Global;
            },
            Token::Ret => {
                let current = current_function.as_mut().expect("Ret outside of function");
 
                current.body.push(AstNode::Ret);

                current_pc += 4;
            },
            Token::EndFn => {
                let current = current_function.take().expect("EndFn without matching Fn");
                functions.push(AstNode::Function {
                    name: current.name.clone(),
                    body: current.body,
                    pc: current.pc,
                    stype
                });

                functions_hashmap.insert(current.name, current.pc);
            },
        }
    }

    if current_function.is_some() {
        return Err(anyhow!("A function has not been closed"));
    }

    Ok((functions, functions_hashmap))
}

