use logos::Logos;
use tracing::error;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex(r"#[a-zA-Z][a-zA-Z0-9_]*", |lex| lex.slice()[1..].to_string())]
    Instruction(String),
}

pub fn get_tokens(input: String) -> Vec<Token> {
    let mut lex = Token::lexer(&input);
    let mut tokens = Vec::new();
    let mut good = true;

    while let Some(token) = lex.next() {
        match token {
            Ok(Token::Instruction(_)) => { tokens.push(token.unwrap()) },
            Err(_) => {
                error!("Invalid Token {}", lex.slice());
                good = false;
            }
        }
    }
    
    if !good {
        std::process::exit(1);
    }

    tokens
}
