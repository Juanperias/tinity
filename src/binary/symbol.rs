use crate::binary::Section;
use crate::parser::ast::AstNode;
use crate::riscv::decode::{from_nodes, DecodeError};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    Global,
    Private,
}

#[derive(Error, Debug)]
pub enum SymbolError {
    #[error("Decode error: {0}")]
    DecodeError(#[from] DecodeError),
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub section: Section,
    pub symbol_type: SymbolType,
    pub content: Vec<u8>,
}

#[derive(Debug)]
pub struct SymbolBuilder {
    pub symbol: Symbol,
}

impl SymbolBuilder {
    pub fn new() -> Self {
        SymbolBuilder {
            symbol: Symbol {
                name: "empty".to_string(),
                symbol_type: SymbolType::Private,
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
    pub fn set_type(mut self, new_type: SymbolType) -> Self {
        self.symbol.symbol_type = new_type;
        self
    }
    #[must_use]
    pub fn from_ast(
        mut self,
        node: &AstNode,
        functions: &HashMap<String, u64>,
    ) -> Result<Self, SymbolError> {
        match node {
            AstNode::Function {
                name, body, stype, ..
            } => {
                self.symbol.name = name.to_string();
                self.symbol.symbol_type = *stype;
                self.symbol.content = from_nodes(body.to_vec(), functions)?;
            }
            _ => {}
        }

        Ok(self)
    }
    pub fn build(self) -> Symbol {
        self.symbol
    }
}
