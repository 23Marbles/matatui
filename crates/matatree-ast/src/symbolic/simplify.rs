use crate::symbolic::SymbolicRepr;

#[derive(Debug, thiserror::Error)]
pub enum SimplifyError {}

impl SymbolicRepr {
    pub fn simplify(&mut self) -> Result<(), SimplifyError> {
        todo!("Implement solving")
    }
}
