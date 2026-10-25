use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use slotmap::{SlotMap, new_key_type};

use crate::layout::tree::{
    container::{Container, ContainerKind, ContainerMut, ContainerRef},
    sequence::{Sequence, SequenceMut, SequenceRef},
};

pub mod container;
pub mod sequence;

new_key_type! {
    pub struct ContainerId;
    pub struct SequenceId;
}

#[derive(Debug, Clone)]
pub struct Tree {
    containers: SlotMap<ContainerId, Container>,
    sequences: SlotMap<SequenceId, Sequence>,
    root: SequenceId,
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

    pub fn root(&self) -> SequenceId {
        self.root
    }

    #[inline(always)]
    pub fn get_container(&self, id: ContainerId) -> Option<ContainerRef<'_>> {
        ContainerRef::get(self, id)
    }

    #[inline(always)]
    pub fn get_mut_container<'a>(&'a mut self, id: ContainerId) -> Option<ContainerMut<'a>> {
        ContainerMut::get(self, id)
    }

    #[inline(always)]
    pub fn get_sequence(&self, id: SequenceId) -> Option<SequenceRef<'_>> {
        SequenceRef::get(self, id)
    }

    #[inline(always)]
    pub fn get_mut_sequence(&mut self, id: SequenceId) -> Option<SequenceMut<'_>> {
        SequenceMut::get(self, id)
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}
