mod binary;
mod parser;
mod riscv;

use anyhow::Result;
use binary::symbol::SymbolBuilder;
use binary::{elf::Elf, Binary, Section};
use object::{Architecture, Endianness};
use parser::ast::get_from_tokens;
use parser::token::get_tokens;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let input = std::fs::read_to_string("test.tir")?;
    let tokens = get_tokens(input)?;
    let ast = get_from_tokens(tokens)?;
    println!("{:#?}", ast);
    let mut elf = Elf::new(Architecture::Riscv64, Endianness::Little);

    let mut f = std::fs::File::create("output.elf")?;
    elf.create_section(Section::Text);

    ast.iter().for_each(|node| {
        let symbol = SymbolBuilder::new().from_ast(&node).build();

        elf.write_section(Section::Text, symbol);
    });

    elf.save(&mut f).unwrap();

    Ok(())
}
