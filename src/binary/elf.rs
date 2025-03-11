use super::symbol;
use super::{Binary, Section};
use object::write::Object;
use object::{
    write::{SectionId, SectionKind},
    Architecture, BinaryFormat, Endianness,
};
use std::fs::File;
use std::io::Write;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ElfError {
    #[error("Io Error {0}")]
    IoError(#[from] std::io::Error),

    #[error("Object Error {0}")]
    ObjectError(#[from] object::write::Error),
}

// High level abstraccion of Object
pub struct Elf<'a> {
    pub object: Object<'a>,
    pub text_id: Option<SectionId>,
    pub current_tvalue: u64,
}

impl<'a> Elf<'a> {
    pub fn new(arch: Architecture, endianness: Endianness) -> Self {
        let obj = Object::new(BinaryFormat::Elf, arch, endianness);

        Self {
            object: obj,
            text_id: None,
            current_tvalue: 0,
        }
    }
    fn wsection(&mut self, section: Section, symbol: symbol::Symbol) {
        match section {
            Section::Text => {
                if let None = self.text_id {
                    self.asection("text".to_string(), SectionKind::Text);
                }

                let text_id = self.text_id.unwrap();

                let (st_info, scope) = if symbol.symbol_type == symbol::SymbolType::Private {
                    (0x10, object::SymbolScope::Compilation)
                } else {
                    (0x12, object::SymbolScope::Linkage)
                };

                self.object.add_symbol(object::write::Symbol {
                    section: object::write::SymbolSection::Section(text_id),
                    name: symbol.name.as_bytes().to_vec(),
                    kind: object::SymbolKind::Text,
                    size: symbol.content.len() as u64,
                    weak: false,
                    value: self.current_tvalue,
                    scope,
                    flags: object::SymbolFlags::Elf {
                        st_info,
                        st_other: 0,
                    },
                });

                self.current_tvalue += symbol.content.len() as u64;

                self.object
                    .section_mut(text_id)
                    .append_data(&symbol.content, 4);
            }
            _ => {}
        }
    }
    fn asection(&mut self, name: String, kind: SectionKind) {
        let n = name.as_bytes().to_vec();

        let id = self.object.add_section(vec![], n, kind);

        self.text_id = Some(id);
    }
}

impl Binary for Elf<'_> {
    type Error = ElfError;

    fn get(&self) -> Result<Vec<u8>, Self::Error> {
        Ok(self.object.write()?)
    }
    fn save(&self, target: &mut File) -> Result<(), Self::Error> {
        let content = self.get()?;
        target.write_all(&content)?;
        Ok(())
    }
    fn write_section(&mut self, section: Section, symbol: symbol::Symbol) {
        self.wsection(section, symbol);
    }
    fn create_section(&mut self, section: Section) {
        match section {
            Section::Text => self.asection("text".to_string(), SectionKind::Text),
            Section::Data => self.asection("data".to_string(), SectionKind::Data),
            Section::Note => self.asection("note".to_string(), SectionKind::Note),
            Section::Bss => self.asection("bss".to_string(), SectionKind::UninitializedData),
            Section::Other(name, id) => self.asection(name, SectionKind::Unknown),
        }
    }
}
