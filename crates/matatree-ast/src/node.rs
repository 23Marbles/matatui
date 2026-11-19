use matatree_data::{Data, numeric::Number};

use crate::node::{mapping::DataMapView, nodeview::{NodeView, NodeViewOwned}, operations::Operation, solver::SolutionError};

pub mod operations;
pub mod value;
pub mod nodeview;
pub mod mapping;
pub mod simplify;
pub mod solver;

pub trait SymbolicNode: Sized {
    /// Disallows approximates, will error on these values
    fn evaluate_strict(&self, mapping: Option<DataMapView>) -> Result<Number, SolutionError> {
        self.as_nodeview().evaluate_strict(mapping)
    }

    fn evaluate(&self, mapping: Option<DataMapView>) -> Result<Number, SolutionError> {
        self.as_nodeview().evaluate(mapping)
    }

    fn into_owned_nodeview(self) -> NodeViewOwned {
        self.to_owned_nodeview()
    }

    fn to_owned_nodeview(&self) -> NodeViewOwned {
        self.as_nodeview().to_owned()
    }

    fn as_nodeview<'a>(&'a self) -> NodeView<'a>;
}

pub trait ValueNode: SymbolicNode {
    fn get_value(&self) -> Data;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeKind {
    Operation(Operation),
    Number(Number),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    kind: NodeKind,
}
