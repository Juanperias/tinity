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
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*:", |lex| {
        lex.slice().replace(":", "").to_string()
    })] 
    Label(String),

    #[token("{")]
    CurlyBracketStart,

    #[token("}")] 
    CurlyBracketEnd,

    #[token("(")]
    ParenthesesStart,

    #[token(")")]
    ParenthesesEnd,


    #[token("define")]
    Define,

    #[regex(r"[A-Za-z_][A-Za-z0-9_]*", |lex| {
        lex.slice().to_string()
    })] 
    Identifier(String),

    #[regex(r"@[A-Za-z_][A-Za-z0-9_]*", |lex| {
        lex.slice().to_string()
    })] 
    GlobalEntity(String)
}

pub fn get_tokens(input: String) -> Result<Vec<Token>, LexerError> {
    let mut lex = Token::lexer(&input);
    let mut tokens = Vec::new();
    let mut has_errors = true;

    while let Some(token) = lex.next() {
        match token {
            Ok(t) => tokens.push(t),
            Err(_) => {
                error!("Invalid Token: {}", lex.slice());
                has_errors = false;
            }
        }
    }

    if !has_errors {
        return Err(LexerError::SyntaxError);
    }

    Ok(tokens)
}
