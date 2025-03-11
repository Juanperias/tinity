pub mod elf;
pub mod symbol;

use object::write::SectionId;
use std::fs::File;
use symbol::Symbol;

#[derive(Debug)]
pub enum Section {
    Text,
    Data,
    Bss,
    Note,
    Other(String, Option<SectionId>),
}

pub trait Binary {
    type Error;

    fn get(&self) -> Result<Vec<u8>, Self::Error>;
    fn write_section(&mut self, section: Section, symbol: Symbol);
    fn create_section(&mut self, section: Section);
    fn save(&self, target: &mut File) -> Result<(), Self::Error>;
}
