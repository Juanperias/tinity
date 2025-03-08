mod binary;
mod parser;
mod riscv;

use parser::ast::get_from_tokens;
use parser::token::get_tokens;
use binary::{elf::Elf, Binary, Section};
use anyhow::Result;
use object::{Architecture, Endianness};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use binary::symbol::SymbolBuilder;

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let input = std::fs::read_to_string("test.tir")?;
    let tokens = get_tokens(input)?;
    let ast = get_from_tokens(tokens)?;
    let mut elf = Elf::new(Architecture::Riscv64, Endianness::Little);

    let mut f = std::fs::File::create("output.elf")?;

    let symbol = SymbolBuilder::new()
        .from_ast(ast[0].clone())
        .build();

    elf.create_section(Section::Text);
    elf.wwsection(symbol);

    elf.save(&mut f).unwrap();

    Ok(())
}
