use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex(r"#[a-zA-Z][a-zA-Z0-9_]*", |lex| lex.slice()[1..].to_string())]
    Instruction(String),
}

pub fn get_tokens() -> Vec<Token> {
    Vec::new()
}
