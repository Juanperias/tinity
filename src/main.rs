mod binary;
mod parser;
mod riscv;

use crate::parser::token::get_tokens;
use binary::{elf::Elf, Binary, Section};
use anyhow::Result;
use object::{Architecture, Endianness};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let input = std::fs::read_to_string("test.tir")?;
    let tokens = get_tokens(input);
    println!("{:?}", tokens);

    let mut elf = Elf::new(Architecture::Riscv64, Endianness::Little);

    let mut f = std::fs::File::create("output.elf")?;

    elf.create_section(Section::Text);
    elf.write_section(Section::Text, vec![]);

    elf.save(&mut f).unwrap();

    Ok(())
}
