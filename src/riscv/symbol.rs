use crate::parser::ast::AstNode;
use crate::binary::Section;
use super::decode::from_nodes;

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
    pub fn set_name(mut self, name: String) -> Self {
        self.symbol.name = name;
        self
    }
    #[must_use]
    pub fn set_section(mut self, section: Section) -> Self {
        self.symbol.section = section;
        self
    } 
    #[must_use]
    pub fn set_content(mut self, content: Vec<u8>) -> Self {
        self.symbol.content = content;
        self
    }
    #[must_use]
    pub fn from_ast(mut self, node: AstNode) -> Self {
        match node {
            AstNode::Function { name, body } => {
                self.symbol.name = name;
                self.symbol.content = from_nodes(body);
            },
            _ => {}
        }

        self
    }
    pub fn build(self) -> Symbol {
        self.symbol
    }
}

