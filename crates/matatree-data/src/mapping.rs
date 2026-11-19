use slotmap::SlotMap;

use crate::{Data, DataKind, identifier::SymbolId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DataSplit {
    kind: DataKind,
    visual_id: String,
}

/// A mapping from the data identifier to the given value
#[derive(Debug, Clone)]
pub struct DataMap {
    mapping: SlotMap<SymbolId, DataSplit>,
}

#[derive(Debug, Clone, Copy)]
pub struct DataMapView<'a> {
    mapping: &'a DataMap,
}

#[derive(Debug)]
pub struct DataMapSetUnknowns<'a> {
    unknowns: SlotMap<SymbolId, &'a mut DataKind>,
}

impl DataMap {
    pub fn as_read_only(&self) -> DataMapView<'_> {
        DataMapView { mapping: &self }
    }
}

impl DataMap {
    pub fn empty() -> Self {
        Self {
            mapping: SlotMap::with_key(),
        }
    }
}
