use std::collections::HashMap;

use matatree_data::Data;

/// A mapping from the data identifier to the given value
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct DataMap {
    mapping: HashMap<String, Data>,
}

pub struct DataMapView<'a> {
    mapping: &'a HashMap<String, Data>,
}

pub struct DataMapSetUnknowns<'a> {
    mapping: &'a mut HashMap<String, Data>,
}

impl DataMap {
    pub fn as_read_only(&self) -> DataMapView<'_> {
        DataMapView {
            mapping: &self.mapping,
        }
    }
}

impl DataMap {
    pub fn empty() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }
}
