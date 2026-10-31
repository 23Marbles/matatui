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

    #[must_use]
    pub fn clear_sequence(&mut self, id: SequenceId) -> Option<OwnedSequence> {
        Some(self.get_mut_sequence(id)?.clear())
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
