use std::{
    array::from_fn,
    ops::{Deref, DerefMut},
};

use super::*;

impl SequenceMut<'_> {
    #[inline]
    pub fn add_atom(&mut self, atom: Atom) {
        self.children.push(Child::Atom(atom));
    }

    // The generic const is the number of `SequenceId`s needed to create `ContainerKind`.
    #[inline]
    pub fn add_container<const S: usize>(
        &mut self,
        kind_func: impl FnOnce([SequenceId; S]) -> ContainerKind,
    ) -> Option<ContainerId> {
        self.insert_container(self.children.len(), kind_func)
    }

    pub fn delete_child(&mut self, idx: usize) -> Option<OwnedChild> {
        if idx >= self.children.len() {
            return None;
        }

        match self.children.remove(idx) {
            Child::Atom(atom) => Some(OwnedChild::Atom(atom)),
            Child::Container(container) => Some(OwnedChild::Container(
                self.tree.get_mut_container(container).unwrap().delete(),
            )),
        }
    }

    #[inline]
    pub fn insert_atom(&mut self, idx: usize, atom: Atom) {
        self.children.insert(idx, Child::Atom(atom));
    }

    pub fn insert_container<const S: usize>(
        &mut self,
        idx: usize,
        kind_func: impl FnOnce([SequenceId; S]) -> ContainerKind,
    ) -> Option<ContainerId> {
        if idx > self.tree.sequences.get(self.id)?.children.len() {
            return None;
        }

        let seq_ids = from_fn(|_| self.tree.sequences.insert(Sequence::new_unchecked(None)));
        let kind = kind_func(seq_ids);
        let id = self
            .tree
            .containers
            .insert(Container::new_unchecked(self.id, kind));

        for seq_id in seq_ids {
            self.tree
                .sequences
                .get_mut(seq_id)
                .expect("Just created this Sequence and &mut Tree has not been passed to any function since then")
                .parent = Some(id)
        }

        self.children.insert(idx, Child::Container(id));
        Some(id)
    }
}

impl<'a> SequenceMut<'a> {
    pub fn tree(&self) -> &Tree {
        self.tree
    }

    pub fn as_ref(&self) -> SequenceRef<'_> {
        self.tree.get_sequence(self.id).unwrap()
    }

    pub fn clear(&mut self) -> OwnedSequence {
        let mut owned = Vec::new();
        let children = std::mem::take(&mut self.children);

        for child in children {
            match child {
                Child::Container(container_id) => {
                    let container = self.tree.get_mut_container(container_id).unwrap().delete();
                    owned.push(OwnedChild::Container(container));
                }
                Child::Atom(atom) => owned.push(OwnedChild::Atom(atom)),
            }
        }

        OwnedSequence(owned)
    }

    pub fn parent(&self) -> Option<ContainerRef<'_>> {
        self.tree.get_container(self.parent_id()?)
    }

    pub fn get(tree: &'a mut Tree, id: SequenceId) -> Option<Self> {
        tree.sequences
            .contains_key(id)
            .then_some(SequenceMut { id, tree })
    }

    pub fn get_child(&self, idx: usize) -> Option<ChildRef<'_>> {
        let child = self.children.get(idx)?;

        match child {
            Child::Container(id) => Some(ChildRef::Container(ContainerRef::get(self.tree, *id)?)),
            Child::Atom(atom) => Some(ChildRef::Atom(atom)),
        }
    }

    pub fn get_child_mut(&mut self, idx: usize) -> Option<ChildMut<'_>> {
        let sequence = self.tree.sequences.get_mut(self.id)?;
        let child = sequence.children.get_mut(idx)?;

        match child {
            Child::Container(id) => Some(ChildMut::Container(self.tree.containers.get_mut(*id)?)),
            Child::Atom(atom) => Some(ChildMut::Atom(atom)),
        }
    }

    pub fn push_child(&mut self, child: Child) {
        self.children.push(child);
    }

    pub fn into_tree_mut(self) -> &'a mut Tree {
        self.tree
    }

    pub fn into_child_mut(self, idx: usize) -> Option<ChildMut<'a>> {
        let sequence = self.tree.sequences.get_mut(self.id)?;
        let child = sequence.children.get_mut(idx)?;

        match child {
            Child::Container(id) => Some(ChildMut::Container(self.tree.containers.get_mut(*id)?)),
            Child::Atom(atom) => Some(ChildMut::Atom(atom)),
        }
    }

    pub fn into_parent_mut(self) -> Option<ContainerMut<'a>> {
        self.tree.get_mut_container(self.parent_id()?)
    }

    pub fn id(&self) -> SequenceId {
        self.id
    }
}

impl DerefMut for SequenceMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.tree.sequences.get_mut(self.id)
            .expect("On construction `Self::id` is guaranteed to be valid and is not invalidated by mutations on `Self::tree`")
    }
}

impl Deref for SequenceMut<'_> {
    type Target = Sequence;

    fn deref(&self) -> &Self::Target {
        self.tree.sequences.get(self.id)
            .expect("On construction `Self::id` is guaranteed to be valid and is not invalidated by mutations on `Self::tree`")
    }
}
