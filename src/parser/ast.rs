use super::token::Token;
use anyhow::{Result, anyhow};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstNode {
    Function {
        name: String,
        body: Vec<AstNode>,
    },
    Sum {
        numbers: Vec<i64>,
        dist: String
    }
}

pub fn get_from_tokens(tokens: Vec<Token>) -> Result<Vec<AstNode>> {
    #[derive(Debug)]
    struct CurrentFunction {
        name: String,
        body: Vec<AstNode>,
    }

    let mut functions = Vec::new();
    let mut current_function: Option<CurrentFunction> = None;

    for token in tokens {
        match token {
            Token::Fn(name) => {
                if current_function.is_some() {
                    return Err(anyhow!(format!("Nested function {} is not allowed", name))); 
                }
                current_function = Some(CurrentFunction {
                    name,
                    body: Vec::new(),
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
            }
            Token::EndFn => {
                let current = current_function.take().expect("EndFn without matching Fn");
                functions.push(AstNode::Function {
                    name: current.name,
                    body: current.body,
                });
            }
        }
    }

    if current_function.is_some() {
        return Err(anyhow!("A function has not been closed"));
    }

    Ok(functions)
}

