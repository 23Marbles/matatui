use crate::node::Node;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Operation {
    Sum { nodes: Vec<Node> },
    Product { nodes: Vec<Node> },
}

pub enum OperationOrder {
    Before,
    After,
    SameTime,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum OperationKind {
    Multiplication = 1,
    Summation = 0,
}

impl Operation {
    pub fn operation_kind(&self) -> OperationKind {
        match self {
            Operation::Sum { .. } => OperationKind::Summation,
            Operation::Product { .. } => OperationKind::Multiplication,
        }
    }

    pub fn operation_order(&self, other: &Self) -> OperationOrder {
        match self.operation_kind().cmp(&other.operation_kind()) {
            std::cmp::Ordering::Less => OperationOrder::After,
            std::cmp::Ordering::Equal => OperationOrder::SameTime,
            std::cmp::Ordering::Greater => OperationOrder::Before,
        }
    }
}
