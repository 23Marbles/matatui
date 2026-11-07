use std::ops::Deref;

use super::*;

impl Sequence {
    pub fn parent_id(&self) -> Option<ContainerId> {
        self.parent
    }
}

impl<'a> SequenceRef<'a> {
    pub fn is_root(&self) -> bool {
        self.tree().root_id() == self.id()
    }

    pub fn id(&self) -> SequenceId {
        self.id
    }

    pub fn tree(&self) -> &Tree {
        self.tree
    }

    pub fn get(tree: &'a Tree, id: SequenceId) -> Option<Self> {
        Some(SequenceRef {
            id,
            seq: tree.sequences.get(id)?,
            tree,
        })
    }

    pub fn get_child(&self, idx: usize) -> Option<ChildRef<'a>> {
        let child = self.seq.children.get(idx)?;

        match child {
            Child::Container(container_id) => Some(ChildRef::Container(ContainerRef::get(
                self.tree,
                *container_id,
            )?)),
            Child::Atom(atom) => Some(ChildRef::Atom(atom)),
        }
    }

    pub fn parent(&self) -> Option<ContainerRef<'_>> {
        self.tree.get_container(self.parent_id()?)
    }

    pub fn children(&self) -> Children<'a> {
        Children::new(*self)
    }

    pub fn child_count(&self) -> usize {
        self.children.len()
    }
}

impl Deref for SequenceRef<'_> {
    type Target = Sequence;

    fn deref(&self) -> &Self::Target {
        self.seq
    }
}
