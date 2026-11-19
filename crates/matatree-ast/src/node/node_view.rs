use std::marker::PhantomData;

pub struct NodeView<'a> {
    _placeholder: PhantomData<&'a ()>,
}

impl NodeView<'_> {
    pub fn to_owned(&self) -> NodeViewOwned {
        NodeViewOwned {}
    }
}

pub struct NodeViewOwned {}

impl NodeViewOwned {
    pub fn as_ref(&self) -> NodeView<'_> {
        NodeView {
            _placeholder: PhantomData,
        }
    }
}
