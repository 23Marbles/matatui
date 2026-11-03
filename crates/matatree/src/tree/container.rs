use std::ops::{Deref, DerefMut};

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ContainerKind {
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

#[derive(Debug, Clone)]
pub enum OwnedContainerKind {
    Fraction {
        numerator: OwnedSequence,
        denominator: OwnedSequence,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Container {
    pub(super) kind: ContainerKind,

    pub(super) parent_id: SequenceId,
}

impl Container {
    pub fn kind(&self) -> &ContainerKind {
        &self.kind
    }

    pub fn parent_id(&self) -> SequenceId {
        self.parent_id
    }

    pub(super) fn new_unchecked(parent_id: SequenceId, kind: ContainerKind) -> Self {
        Self { parent_id, kind }
    }
}

#[derive(Debug, Copy)]
pub struct ContainerRef<'a> {
    id: ContainerId,

    tree: &'a Tree,

    container: &'a Container,
}

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

pub struct ContainerMut<'a> {
    /// On construction this is guaranteed to exist
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

    pub fn tree(&self) -> &Tree {
        self.tree
    }

    pub fn parent(&self) -> Option<SequenceRef<'_>> {
        self.tree.get_sequence(self.parent_id())
    }

    pub fn delete(self) -> OwnedContainer {
        OwnedContainer(match self.kind {
            ContainerKind::Fraction {
                numerator,
                denominator,
            } => {
                let parent_id = self.parent_id();

                let tree = self.tree;

                let kind = OwnedContainerKind::Fraction {
                    numerator: tree.get_mut_sequence(numerator).unwrap().clear(),
                    denominator: tree.get_mut_sequence(denominator).unwrap().clear(),
                };

                tree.sequences.remove(numerator);
                tree.sequences.remove(denominator);

                let id = self.id;

                let mut parent = tree.get_mut_sequence(parent_id).unwrap();

                for (i, ch) in parent.children.iter().enumerate() {
                    if ch == &Child::Container(id) {
                        parent.children.remove(i);
                        break;
                    }
                }

                tree.containers.remove(id);

                kind
            }
        })
    }

    pub fn into_tree_mut(self) -> &'a mut Tree {
        self.tree
    }
    pub fn into_parent_mut(self) -> Option<SequenceMut<'a>> {
        self.tree.get_mut_sequence(self.parent_id())
    }
}

impl DerefMut for ContainerMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.tree.containers.get_mut(self.id)
            .expect("On construction `Self::id` is guaranteed to be valid and is not invalidated by mutations on `Self::tree`")
    }
}

impl Deref for ContainerMut<'_> {
    type Target = Container;

    fn deref(&self) -> &Self::Target {
        self.tree.containers.get(self.id)
            .expect("On construction `Self::id` is guaranteed to be valid and is not invalidated by mutations on `Self::tree`")
    }
}
#[derive(Debug, Clone)]
pub struct OwnedContainer(pub OwnedContainerKind);
