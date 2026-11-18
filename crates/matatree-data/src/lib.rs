use num_rational::BigRational;

pub mod numeric;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum DataKind {
    Rational(BigRational),
    Number(),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Data {
    kind: Option<DataKind>,
}

impl Data {
    pub fn kind(&self) -> Option<&DataKind> {
        self.kind.as_ref()
    }
}
