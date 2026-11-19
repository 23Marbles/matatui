use slotmap::SlotMap;

use crate::{
    identifier::{SymbolId, SymbolicIdent},
    numeric::Number,
};

pub mod identifier;
pub mod mapping;
pub mod numeric;

/// Data is what is stored in variables,
/// so this can be any type of data that can be stored in a variables
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum DataKind {
    Number(Number),
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Data {
    kind: DataKind,
    ident: SymbolicIdent,
}

impl Data {
    pub fn kind(&self) -> &DataKind {
        &self.kind
    }
}
