use crate::node::Node;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Operation {
    Sum {
        nodes: Vec<Node>,
    },
    Product {
        nodes: Vec<Node>,
    },
    Division {
        numerator: Box<Node>,
        denominator: Box<Node>,
    },
    Subtraction {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

pub enum OperationOrder {
    Before,
    After,
    SameTime,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum OperationKind {
    Multiplication,
    Division,
    Summation,
    Subtraction,
}

impl OperationKind {
    /// When self should be evaluated in reference to other
    pub fn order(&self, other: Self) -> OperationOrder {
        match (self, other) {
            (
                OperationKind::Subtraction | OperationKind::Summation,
                OperationKind::Division | OperationKind::Multiplication,
            ) => OperationOrder::After,
            (
                OperationKind::Division | OperationKind::Multiplication,
                OperationKind::Division | OperationKind::Multiplication,
            )
            | (
                OperationKind::Subtraction | OperationKind::Summation,
                OperationKind::Subtraction | OperationKind::Summation,
            ) => OperationOrder::SameTime,
            (
                OperationKind::Division | OperationKind::Multiplication,
                OperationKind::Subtraction | OperationKind::Summation,
            ) => OperationOrder::After,
        }
    }
}

impl Operation {
    pub fn operation_kind(&self) -> OperationKind {
        match self {
            Operation::Sum { .. } => OperationKind::Summation,
            Operation::Product { .. } => OperationKind::Multiplication,
            Operation::Division { .. } => OperationKind::Division,
            Operation::Subtraction { .. } => OperationKind::Subtraction,
        }
    }

    /// When self should be evaluated in reference to other
    pub fn operation_order(&self, other: &Self) -> OperationOrder {
        self.operation_kind().order(other.operation_kind())
    }
}
