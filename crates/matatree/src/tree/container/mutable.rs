use std::ops::{Deref, DerefMut};

use super::*;

impl<'a> ContainerMut<'a> {
    pub fn get(tree: &'a mut Tree, id: ContainerId) -> Option<Self> {
        tree.containers
            .contains_key(id)
            .then_some(ContainerMut { id, tree })
    }

    pub fn as_ref(&self) -> ContainerRef<'_> {
        self.tree.get_container(self.id).unwrap()
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

    pub fn into_parent_mut(self) -> SequenceMut<'a> {
        self.tree.get_mut_sequence(self.parent_id()).unwrap()
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
