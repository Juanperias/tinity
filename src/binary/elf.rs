use object::write::Object;
use object::{BinaryFormat, Endianness, Architecture, write::{SectionKind, SectionId}};
use super::{Binary, Section};
use anyhow::Result;
use std::fs::File;
use std::io::Write;

// High level abstraccion of Object
pub struct Elf<'a> {
    pub object: Object<'a>,
    pub text_id: Option<SectionId>
}

fn create_text_section() -> Vec<u8> {
    let opcode = 0x13;
    let rd = 0x0A;  
    let funct3 = 0x00;  
    let rs1 = 0x0A;
    let imm = 30;   
    let instruction = (imm as u32) << 20
        | (rs1 as u32) << 15               
        | (funct3 as u32) << 12     
        | (rd as u32) << 7          
        | opcode as u32;                
    instruction.to_le_bytes().to_vec()
}


impl<'a> Elf<'a> {
    pub fn new(arch: Architecture, endianness: Endianness) -> Self {
        let obj = Object::new(BinaryFormat::Elf, arch, endianness);
        
        Self {
            object: obj,
            text_id: None
        }
    }
    fn wsection(&mut self, content: Vec<u8>, section: Section) {
        match section {
            Section::Text => { 
                if let None = self.text_id {
                    self.asection("text".to_string(), SectionKind::Text);
                }

                let text_id = self.text_id.unwrap();

                let n = b"_start".to_vec();
                let content = create_text_section();

                self.object.add_symbol(object::write::Symbol {
                    section: object::write::SymbolSection::Section(text_id),
                    name: n,
                    kind: object::SymbolKind::Text,
                    size: content.len() as u64,
                    weak: false,
                    value: 0,
                    scope: object::SymbolScope::Linkage,
                    flags: object::SymbolFlags::Elf {
                        st_info: 0x12,
                        st_other: 0,
                    }
                });
                self.object.section_mut(text_id).append_data(&content, std::mem::align_of_val(&content).try_into().unwrap());
            },
            _ => {}
        }
    }
    fn asection(&mut self, name: String, kind: SectionKind) {
        let n = name.as_bytes().to_vec();

        let id = self.object.add_section(vec![], n, kind);
        
        self.text_id = Some(id);}
}


impl Binary for Elf<'_> {
    fn get(&self) -> Result<Vec<u8>> {
        Ok(self.object.write()?)
    }
    fn save(&self, target: &mut File) -> Result<()> {
        let content = self.get()?;
        target.write_all(&content)?;
        Ok(())
    }
    fn write_section(&mut self, section: Section, content: Vec<u8>) {
        self.wsection(content, section);  
    }
    fn create_section(&mut self, section: Section) {
        match section {
            Section::Text => { self.asection("text".to_string(), SectionKind::Text) },
            Section::Data => { self.asection("data".to_string(), SectionKind::Data) },
            Section::Note => { self.asection("note".to_string(), SectionKind::Note) },
            Section::Bss => { self.asection("bss".to_string(), SectionKind::UninitializedData) },
            Section::Other(name, id) => { self.asection(name, SectionKind::Unknown) }
        }
    }
}
