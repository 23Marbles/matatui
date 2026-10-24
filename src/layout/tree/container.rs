use std::cmp::max;

use super::*;

#[derive(Clone, Debug)]
pub(super) enum ContainerKind {
    Fraction {
        numerator: SequenceId,
        denominator: SequenceId,
    },
}

impl ContainerKind {
    pub fn fraction(seq_ids: [SequenceId; 2]) -> Self {
        Self::Fraction {
            numerator: seq_ids[0],
            denominator: seq_ids[1],
        }
    }
}

pub struct Container {
    parent_id: SequenceId,
    kind: ContainerKind,
}

impl Container {
    pub(super) fn new_unchecked(parent_id: SequenceId, kind: ContainerKind) -> Self {
        Self { parent_id, kind }
    }

    pub fn parent_id(&self) -> SequenceId {
        self.parent_id
    }

    pub(super) fn kind(&self) -> &ContainerKind {
        &self.kind
    }

    pub fn x_size(&self, tree: &Tree) -> Option<usize> {
        match self.kind {
            ContainerKind::Fraction {
                numerator,
                denominator,
            } => max(
                tree.get_sequence(numerator)?.x_size(tree),
                tree.get_sequence(denominator)?.x_size(tree),
            ),
        }
    }
}

pub struct ContainerRef<'a>(&'a Container, &'a Tree);

impl<'a> ContainerRef<'a> {
    pub fn get(tree: &'a Tree, id: ContainerId) -> Option<Self> {
        Some(ContainerRef(tree.containers.get(id)?, tree))
    }

    pub fn parent(&self) -> Option<SequenceRef<'_>> {
        self.1.get_sequence(self.parent_id())
    }

    pub fn tree(&self) -> &Tree {
        self.1
    }
}

impl Deref for ContainerRef<'_> {
    type Target = Container;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

pub struct ContainerMut<'a> {
    /// On construction this is guarunteed to exist
    id: ContainerId,
    /// Mutating tree must not allow `Self::id` to become invalid
    tree: &'a mut Tree,
}

impl<'a> ContainerMut<'a> {
    pub fn get(tree: &'a mut Tree, id: ContainerId) -> Option<Self> {
        tree.containers
            .contains_key(id)
            .then_some(ContainerMut { id, tree })
    }

    pub fn parent(&self) -> Option<SequenceRef<'_>> {
        self.tree.get_sequence(self.parent_id())
    }

    pub fn tree(&self) -> &Tree {
        self.tree
    }

    pub fn into_parent_mut(self) -> Option<SequenceMut<'a>> {
        self.tree.get_mut_sequence(self.parent_id())
    }

    pub fn into_tree_mut(self) -> &'a mut Tree {
        self.tree
    }
}

impl DerefMut for ContainerMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.tree.containers.get_mut(self.id)
            .expect("On construction `Self::id` is guarunteed to be valid and is not invalidated by mutations on `Self::tree`")
    }
}

impl Deref for ContainerMut<'_> {
    type Target = Container;

    fn deref(&self) -> &Self::Target {
        self.tree.containers.get(self.id)
            .expect("On construction `Self::id` is guarunteed to be valid and is not invalidated by mutations on `Self::tree`")
    }
}
