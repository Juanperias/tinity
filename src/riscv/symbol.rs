use crate::parser::ast::AstNode;
use crate::binary::Section;

#[derive(Debug)]
pub enum SymbolFlags {}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub section: Section,
    //pub flags: SymbolFlags,
    pub content: Vec<u8>
}

#[derive(Debug)]
pub struct SymbolBuilder {
    pub symbol: Symbol
}

impl SymbolBuilder {
    pub fn new() -> Self {
        SymbolBuilder {
            symbol: Symbol {
                name: "empty".to_string(),
                content: Vec::new(),
                section: Section::Note,
            },
        }
    }
    #[must_use]
    pub fn set_name(mut self, name: String) {
        self.symbol.name = name;
    }
    #[must_use]
    pub fn set_section(mut self, section: Section) {
        self.symbol.section = section;
    } 
    #[must_use]
    pub fn set_content(mut self, content: Vec<u8>) {
        self.symbol.content = content;
    }
    pub fn build(self) -> Symbol {
        self.symbol
    }
}

