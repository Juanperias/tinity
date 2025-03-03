pub mod elf;
use std::fs::File;
use anyhow::Result;
use object::write::SectionId;

#[derive(Debug)]
pub enum Section {
    Text,
    Data,
    Bss,
    Note,
    Other(String, Option<SectionId>)
}

pub trait Binary {
    fn get(&self) -> Result<Vec<u8>>;
    fn write_section(&mut self, section: Section, content: Vec<u8>);
    fn create_section(&mut self, section: Section);
    fn save(&self, target: &mut File) -> Result<()>; 
}
