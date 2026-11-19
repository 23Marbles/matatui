use crate::numeric::Number;

pub mod numeric;

/// Data is what is stored in variables,
/// so this can be any type of data that can be stored in a variables
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum DataKind {
    Number(Number),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Data {
    kind: DataKind,
}

impl Data {
    pub fn kind(&self) -> &DataKind {
        &self.kind
    }
}
