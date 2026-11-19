use crate::node::node_view::NodeViewOwned;

#[derive(Debug, thiserror::Error)]
pub enum SimplifyError {}

impl NodeViewOwned {
    pub fn simplify(&mut self) -> Result<(), SimplifyError> {
        todo!("Implement solving")
    }
}
