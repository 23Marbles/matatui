use std::array::from_fn;

use unicode_segmentation::UnicodeSegmentation;

use crate::layout::tree::sequence::child::{Child, ChildMut, ChildRef};

use super::*;

pub struct Sequence {
    parent: Option<ContainerId>,
    children: Vec<Child>,
}

pub mod child {
    use super::*;

    #[derive(Clone, Debug)]
    pub enum Child {
        Container(ContainerId),
        Text(String),
    }

    impl Child {
        pub fn x_size(&self, tree: &Tree) -> Option<usize> {
            match self {
                Child::Container(container_id) => {
                    tree.get_container(*container_id).map(|c| c.x_size(tree))?
                }
                Child::Text(s) => Some(s.graphemes(true).count()),
            }
        }
    }

    pub enum ChildRef<'a> {
        Container(&'a Container),
        Text(&'a str),
    }

    pub enum ChildMut<'a> {
        Container(&'a mut Container),
        Text(&'a mut String),
    }
}

impl Tree {
    /// The generic const is the number of `SequenceId`s that are needed in order to create the `ContainerKind`
    #[inline]
    pub(super) fn add_child<const S: usize>(
        &mut self,
        parent: SequenceId,
        kind_func: impl FnOnce([SequenceId; S]) -> ContainerKind,
    ) -> Option<ContainerId> {
        let idx = self.get_sequence(parent)?.children.len();

        self.insert_child(idx, parent, kind_func)
    }

    pub(super) fn insert_child<const S: usize>(
        &mut self,
        idx: usize,
        parent_id: SequenceId,
        kind_func: impl FnOnce([SequenceId; S]) -> ContainerKind,
    ) -> Option<ContainerId> {
        if idx > self.sequences.get(parent_id)?.children.len() {
            return None;
        }

        let seq_ids = from_fn(|_| self.sequences.insert(Sequence::new_unchecked(None)));

        let kind = kind_func(seq_ids);

        let id = self
            .containers
            .insert(Container::new_unchecked(parent_id, kind));

        for seq_id in seq_ids {
            self.sequences
                .get_mut(seq_id)
                .expect("Just created this Sequence and &mut Tree has not been passed to any function since then")
                .parent = Some(id)
        }

        let parent = self.sequences.get_mut(parent_id)?;

        parent.children.insert(idx, Child::Container(id));

        Some(id)
    }
}

impl Sequence {
    pub(super) fn new_unchecked(parent: Option<ContainerId>) -> Self {
        Self {
            parent,
            children: Vec::new(),
        }
    }

    pub fn parent_id(&self) -> Option<ContainerId> {
        self.parent
    }

    pub fn x_size(&self, tree: &Tree) -> Option<usize> {
        let mut sum = 0;

        for c_size in self.children.iter().map(|c| c.x_size(tree)) {
            sum += c_size?;
        }

        Some(sum)
    }

    pub fn x_size_before_index(&self, tree: &Tree, index: usize) -> Option<usize> {
        if index > self.children.len() {
            return None;
        }

        self.children[..index]
            .iter()
            .map(|child| child.x_size(tree))
            .sum()
    }

    pub fn x_size_before_container(&self, tree: &Tree, container_id: ContainerId) -> Option<usize> {
        let mut sum = 0;

        for child in &self.children {
            if matches!(child, Child::Container(id) if *id == container_id) {
                return Some(sum);
            }

            sum += child.x_size(tree)?;
        }

        None
    }
}

pub struct SequenceRef<'a> {
    seq: &'a Sequence,
    tree: &'a Tree,
}

impl<'a> SequenceRef<'a> {
    pub fn get(tree: &'a Tree, id: SequenceId) -> Option<Self> {
        Some(SequenceRef {
            seq: tree.sequences.get(id)?,
            tree,
        })
    }

    pub fn get_child(&self, idx: usize) -> Option<ChildRef<'a>> {
        let child = self.seq.children.get(idx)?;

        match child {
            Child::Container(container_id) => Some(ChildRef::Container(
                self.tree.containers.get(*container_id)?,
            )),
            Child::Text(s) => Some(ChildRef::Text(s)),
        }
    }

    pub fn parent(&self) -> Option<ContainerRef<'_>> {
        self.tree.get_container(self.parent_id()?)
    }

    pub fn tree(&self) -> &Tree {
        self.tree
    }
}

impl Deref for SequenceRef<'_> {
    type Target = Sequence;

    fn deref(&self) -> &Self::Target {
        self.seq
    }
}

pub struct SequenceMut<'a> {
    /// On construction this is guaranteed to exist
    id: SequenceId,
    /// Mutating tree must not allow `Self::id` to become invalid
    tree: &'a mut Tree,
}

impl<'a> SequenceMut<'a> {
    pub fn get(tree: &'a mut Tree, id: SequenceId) -> Option<Self> {
        tree.sequences
            .contains_key(id)
            .then_some(SequenceMut { id, tree })
    }

    pub fn get_child(&self, idx: usize) -> Option<ChildRef<'_>> {
        let child = self.children.get(idx)?;

        match child {
            Child::Container(id) => Some(ChildRef::Container(self.tree.containers.get(*id)?)),
            Child::Text(text) => Some(ChildRef::Text(text)),
        }
    }

    pub fn get_child_mut(&mut self, idx: usize) -> Option<ChildMut<'_>> {
        let sequence = self.tree.sequences.get_mut(self.id)?;
        let child = sequence.children.get_mut(idx)?;

        match child {
            Child::Container(id) => Some(ChildMut::Container(self.tree.containers.get_mut(*id)?)),
            Child::Text(text) => Some(ChildMut::Text(text)),
        }
    }

    pub fn into_child_mut(self, idx: usize) -> Option<ChildMut<'a>> {
        let sequence = self.tree.sequences.get_mut(self.id)?;
        let child = sequence.children.get_mut(idx)?;

        match child {
            Child::Container(id) => Some(ChildMut::Container(self.tree.containers.get_mut(*id)?)),
            Child::Text(text) => Some(ChildMut::Text(text)),
        }
    }

    pub fn parent(&self) -> Option<ContainerRef<'_>> {
        self.tree.get_container(self.parent_id()?)
    }

    pub fn into_parent_mut(self) -> Option<ContainerMut<'a>> {
        self.tree.get_mut_container(self.parent_id()?)
    }

    pub fn tree(&self) -> &Tree {
        self.tree
    }

    pub fn into_tree_mut(self) -> &'a mut Tree {
        self.tree
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
