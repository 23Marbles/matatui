mod child;
mod container;
mod sequence;

use slotmap::SlotMap;

pub use child::*;
pub use container::*;
pub use sequence::*;

slotmap::new_key_type! {
    pub struct ContainerId;
    pub struct SequenceId;
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tree {
    root: SequenceId,

    sequences: SlotMap<SequenceId, Sequence>,

    containers: SlotMap<ContainerId, Container>,
}

impl Tree {
    pub fn new() -> Self {
        let mut sequences = SlotMap::with_key();

        let root = sequences.insert(Sequence::new_unchecked(None));

        Self {
            sequences,
            containers: SlotMap::with_key(),
            root,
        }
    }

    pub fn root(&self) -> SequenceRef<'_> {
        self.get_sequence(self.root)
            .expect("Root should not be removed at any point")
    }
    pub fn root_id(&self) -> SequenceId {
        self.root
    }

    pub fn clear_sequence(&mut self, id: SequenceId) -> Option<OwnedSequence> {
        Some(self.get_mut_sequence(id)?.clear())
    }

    pub fn delete_container(&mut self, id: ContainerId) -> Option<OwnedContainer> {
        Some(self.get_mut_container(id)?.delete())
    }

    #[inline(always)]
    pub fn get_sequence(&self, id: SequenceId) -> Option<SequenceRef<'_>> {
        SequenceRef::get(self, id)
    }
    #[inline(always)]
    pub fn get_container(&self, id: ContainerId) -> Option<ContainerRef<'_>> {
        ContainerRef::get(self, id)
    }
    #[inline(always)]
    pub fn get_mut_sequence(&mut self, id: SequenceId) -> Option<SequenceMut<'_>> {
        SequenceMut::get(self, id)
    }
    #[inline(always)]
    pub fn get_mut_container<'a>(&'a mut self, id: ContainerId) -> Option<ContainerMut<'a>> {
        ContainerMut::get(self, id)
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slotmap::Key;

    #[test]
    fn new_tree_has_an_empty_root_sequence() {
        let tree = Tree::new();
        let root = tree.root();

        assert_eq!(root.id(), tree.root_id());
        assert_eq!(root.parent_id(), None);
        assert_eq!(root.child_count(), 0);
        assert!(tree.get_sequence(SequenceId::null()).is_none());
    }

    #[test]
    fn deleting_container_removes_its_sequences_and_parent_child() {
        let mut tree = Tree::new();
        let (container_id, numerator, denominator) = {
            let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
            let container_id = root
                .add_container(|[numerator, denominator]| {
                    ContainerKind::fraction([numerator, denominator])
                })
                .unwrap();
            let ContainerKind::Fraction {
                numerator,
                denominator,
            } = root
                .tree()
                .get_container(container_id)
                .unwrap()
                .kind()
                .clone();
            (container_id, numerator, denominator)
        };

        tree.get_mut_sequence(numerator)
            .unwrap()
            .add_atom(Atom::Text("top".into()));
        tree.get_mut_sequence(denominator)
            .unwrap()
            .add_atom(Atom::Text("bottom".into()));

        let owned = tree.delete_container(container_id).unwrap();

        assert!(tree.get_container(container_id).is_none());
        assert!(tree.get_sequence(numerator).is_none());
        assert!(tree.get_sequence(denominator).is_none());
        assert_eq!(tree.root().child_count(), 0);
        assert!(matches!(
            owned.0,
            OwnedContainerKind::Fraction {
                numerator: OwnedSequence(ref children),
                denominator: OwnedSequence(ref bottom_children),
            } if matches!(children.as_slice(), [OwnedChild::Atom(Atom::Text(top))] if top == "top")
                && matches!(bottom_children.as_slice(), [OwnedChild::Atom(Atom::Text(bottom))] if bottom == "bottom")
        ));
    }

    #[test]
    fn clearing_sequence_deletes_nested_containers() {
        let mut tree = Tree::new();
        let container_id = tree
            .get_mut_sequence(tree.root_id())
            .unwrap()
            .add_container(|[numerator, denominator]| {
                ContainerKind::fraction([numerator, denominator])
            })
            .unwrap();

        let OwnedSequence(children) = tree.clear_sequence(tree.root_id()).unwrap();

        assert!(matches!(children.as_slice(), [OwnedChild::Container(_)]));
        assert!(tree.get_container(container_id).is_none());
        assert_eq!(tree.root().child_count(), 0);
    }
}
