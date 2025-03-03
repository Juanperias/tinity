mod parser;
mod binary;

use tracing::Level;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use crate::parser::token::get_tokens;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Cannot set default subscriber");

    let input = std::fs::read_to_string("test.tir").unwrap();
    let tokens = get_tokens(input);
    println!("{:?}", tokens); 
}
