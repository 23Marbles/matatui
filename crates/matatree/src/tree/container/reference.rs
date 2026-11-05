use std::ops::Deref;

use super::*;

impl<'a> ContainerRef<'a> {
    pub fn id(&self) -> ContainerId {
        self.id
    }

    pub fn get(tree: &'a Tree, id: ContainerId) -> Option<Self> {
        Some(ContainerRef {
            id,
            container: tree.containers.get(id)?,
            tree,
        })
    }

    pub fn tree(&self) -> &'a Tree {
        self.tree
    }

    pub fn parent(&self) -> Option<SequenceRef<'_>> {
        self.tree.get_sequence(self.parent_id())
    }
}

impl Clone for ContainerRef<'_> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            tree: self.tree,
            container: self.container,
        }
    }
}

impl PartialEq for ContainerRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.tree.root_id() == other.tree.root_id()
            && self.container == other.container
    }
}

impl Deref for ContainerRef<'_> {
    type Target = Container;

    fn deref(&self) -> &Self::Target {
        self.container
    }
}
