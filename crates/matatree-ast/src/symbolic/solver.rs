use matatree_data::numeric::Number;

use crate::symbolic::{
    SymbolicReprRef,
    mapping::{DataMap, DataMapSetUnknowns, DataMapView},
};

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone, Hash)]
pub enum SolutionError {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Solution {
    complete: bool,
    data_map: DataMap,
}

impl SymbolicReprRef<'_> {
    pub fn solve_variables(&self, variables: DataMapSetUnknowns) -> Result<(), SolutionError> {
        let _ = variables;

        todo!("Implement solving")
    }

    /// Disallows approximates, will error on these values
    pub fn evaluate_strict(&self, mapping: Option<DataMapView>) -> Result<Number, SolutionError> {
        let _ = mapping;

        todo!("Implement solving")
    }

    pub fn evaluate(&self, mapping: Option<DataMapView>) -> Result<Number, SolutionError> {
        let _ = mapping;

        todo!("Implement solving")
    }
}
