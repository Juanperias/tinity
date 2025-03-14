use super::types::Type;
use crate::type_from_string;
use logos::Logos;
use std::fmt::Display;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum LexerError {
    SyntaxError,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex(r"@fn\s+([a-zA-Z_][a-zA-Z0-9_]*)", |lex| {
        let last = lex.slice();
        last.split_whitespace().nth(1).unwrap().to_string()
    })]
    Fn(String),

    #[regex(r"@sum\s+([a-zA-Z_][a-zA-Z0-9_]*|\d+),\s*(%?[a-zA-Z_][a-zA-Z0-9_]*|\d+)((?:,\s*(-?\d+|%[a-zA-Z_][a-zA-Z0-9_]*))+)", |lex| {  
        let last: String = lex.slice().replace(" ", "").chars().skip(4).collect();
        let parts: Vec<&str> = last.split(',').collect();
        let t = parts[0];
        let dist = parts[1].to_string().replace("%", "");

        let mut numbers = Vec::new();

        for part in parts.into_iter().skip(2) {
            numbers.push(type_from_string!(t, part));
        }

        (dist, numbers, t.to_string())
    })]
    Sum((String, Vec<Type>, String)),

    #[regex(r"@go\s+([a-zA-Z_][a-zA-Z0-9_]*)", |lex| {
        let last = lex.slice();
        last.split_whitespace().nth(1).unwrap().to_string()
    })]
    Go(String),

    #[token("@ret")]
    Ret,

    #[token("$global")]
    Global,

    #[regex(
        r"@load\s+%(\w+)\s*,\s*(-?\d+)", 
        |lex| {
            let input: String = lex.slice().replace(" ", "").chars().skip(6).collect();
            let parts = input.split(',');
            let params: Vec<&str> = parts.collect();

            (params[0].to_string(), params[1].parse::<i64>().unwrap())
        }
    )]
    Load((String, i64)),

    // Register Add
    #[regex(
        r"@radd\s+%(\w+)\s*,\s+%(-?\w+)", 
        |lex| {
            let input: String = lex.slice().replace(" ", "").chars().skip(6).collect();
            let parts = input.split(',');
            let params: Vec<&str> = parts.collect();

            (params[0].to_string(), params[1].to_string().replace("%", ""))            
        }
    )]
    Radd((String, String)),

    // Register Sub
    #[regex(
        r"@rsub\s+%(\w+)\s*,\s+%(-?\w+)", 
        |lex| {
            let input: String = lex.slice().replace(" ", "").chars().skip(6).collect();
            let parts = input.split(',');
            let params: Vec<&str> = parts.collect();

            (params[0].to_string(), params[1].to_string().replace("%", ""))            
        }
    )]
    Rsub((String, String)),

    #[token("@syscall")]
    Syscall,

    #[token("@nop")]
    Nop,

    #[token("@endfn")]
    EndFn,
}

pub fn get_tokens(input: String) -> Result<Vec<Token>, LexerError> {
    let mut lex = Token::lexer(&input);
    let mut tokens = Vec::new();
    let mut has_errors = true;

    while let Some(token) = lex.next() {
        match token {
            Ok(t) => tokens.push(t),
            Err(_) => {
                error!("Invalid Token {}", lex.slice());
                has_errors = false;
            }
        }
    }

    if !has_errors {
        return Err(LexerError::SyntaxError);
    }

    Ok(tokens)
}
