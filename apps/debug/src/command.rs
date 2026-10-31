use matatree::tree::{Atom, ContainerId, ContainerKind, SequenceId};

use crate::app::DebugApp;

pub enum TreeEdit {
    AddAtom {
        atom: Atom,

        parent: SequenceId,
    },
    AddFraction {
        parent: SequenceId,
    },

    ReplaceAtom {
        atom: Atom,

        index: usize,

        parent: SequenceId,
    },

    InsertAtom {
        atom: Atom,

        index: usize,

        parent: SequenceId,
    },
    InsertFraction {
        index: usize,

        parent: SequenceId,
    },

    DeleteChild {
        index: usize,

        parent: SequenceId,
    },
    DeleteContainer {
        id: ContainerId,
    },
}

impl DebugApp {
    pub fn apply_edit(&mut self, edit: TreeEdit) {
        match edit {
            TreeEdit::InsertFraction { parent, index } => {
                self.tree
                    .get_mut_sequence(parent)
                    .unwrap()
                    .insert_container(index, ContainerKind::fraction)
                    .expect("Failed to create child");
            }
            TreeEdit::AddFraction { parent } => {
                self.tree
                    .get_mut_sequence(parent)
                    .unwrap()
                    .add_container(ContainerKind::fraction)
                    .expect("Failed to create child");
            }
            TreeEdit::DeleteContainer { id } => {
                self.tree.get_mut_container(id).unwrap().delete();
            }
            TreeEdit::AddAtom { atom, parent } => {
                self.tree.get_mut_sequence(parent).unwrap().add_atom(atom)
            }
            TreeEdit::InsertAtom {
                parent,
                index,
                atom,
            } => self
                .tree
                .get_mut_sequence(parent)
                .unwrap()
                .insert_atom(index, atom),
            TreeEdit::ReplaceAtom {
                parent,
                index,
                atom,
            } => {
                match self
                    .tree
                    .get_mut_sequence(parent)
                    .unwrap()
                    .get_child_mut(index)
                    .unwrap()
                {
                    matatree::tree::ChildMut::Atom(a) => *a = atom,
                    matatree::tree::ChildMut::Container(_) => {
                        log::error!("Tried to replace an atom but found a contianer")
                    }
                };
            }
            TreeEdit::DeleteChild { parent, index } => assert!(
                self.tree
                    .get_mut_sequence(parent)
                    .unwrap()
                    .delete_child(index)
                    .is_some()
            ),
        }
    }
}
