use matatree_data::{Data, numeric::Number};

use crate::{
    node::operations::Operation,
    symbolic::{SymbolicRepr, SymbolicReprRef, mapping::DataMapView, solver::SolutionError},
};

pub mod operations;
pub mod value;

pub trait SymbolicNode: Sized {
    /// Disallows approximates, will error on these values
    fn evaluate_strict(&self, mapping: Option<DataMapView>) -> Result<Number, SolutionError> {
        self.as_symbolic_repr().evaluate_strict(mapping)
    }

    fn evaluate(&self, mapping: Option<DataMapView>) -> Result<Number, SolutionError> {
        self.as_symbolic_repr().evaluate(mapping)
    }

    fn into_symbolic_repr(self) -> SymbolicRepr {
        self.to_symbolic_repr()
    }

    fn to_symbolic_repr(&self) -> SymbolicRepr {
        self.as_symbolic_repr().to_owned()
    }

    fn as_symbolic_repr<'a>(&'a self) -> SymbolicReprRef<'a>;
}

pub trait ValueNode: SymbolicNode {
    fn get_value(&self) -> Data;
}

pub enum NodeKind {
    Operation(Operation),
    Number(Number),
}

pub struct Node {
    kind: NodeKind,
}
