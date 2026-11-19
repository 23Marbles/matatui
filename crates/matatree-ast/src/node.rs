use matatree_data::{Data, numeric::Number};

use crate::node::{
    mapping::DataMapView,
    node_view::{NodeView, NodeViewOwned},
    operations::Operation,
    solver::SolutionError,
};

pub mod node_view;
pub mod operations;
pub mod simplify;
pub mod solver;

pub trait SymbolicNode: Sized {
    /// Disallows approximates, will error on these values
    fn evaluate_strict(&self, mapping: Option<DataMapView>) -> Result<Number, SolutionError> {
        self.as_node_view().evaluate_strict(mapping)
    }

    fn evaluate(&self, mapping: Option<DataMapView>) -> Result<Number, SolutionError> {
        self.as_node_view().evaluate(mapping)
    }

    fn into_owned_node_view(self) -> NodeViewOwned {
        self.to_owned_node_view()
    }

    fn to_owned_node_view(&self) -> NodeViewOwned {
        self.as_node_view().to_owned()
    }

    fn as_node_view<'a>(&'a self) -> NodeView<'a>;
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
