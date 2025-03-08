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
use clap::Parser;

#[derive(Parser)]
struct Args {
    file: String,

    #[clap(short, long)]
    output: Option<String>
}

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let args = Args::parse();

    let input = std::fs::read_to_string(args.file)?;
    let tokens = get_tokens(input)?;
    let (ast, functions) = get_from_tokens(tokens)?;

    let mut elf = Elf::new(Architecture::Riscv64, Endianness::Little);

    let output = args.output.unwrap_or("output.elf".to_string());

    let mut f = std::fs::File::create(output)?;
    elf.create_section(Section::Text);

    ast.iter().for_each(|node| {
        let builder = SymbolBuilder::new().from_ast(&node, &functions);

        if let Ok(builder) = builder {
            let symbol = builder.build();
            elf.write_section(Section::Text, symbol);
        } else if let Err(e) = builder {
            eprintln!("Error processing AST: {}", e);
        }
    });

    elf.save(&mut f).unwrap();

    Ok(())
}
