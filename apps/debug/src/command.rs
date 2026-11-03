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
    Left,
    Right,
}

impl DebugApp {
    pub fn apply_edit(&mut self, edit: TreeEdit) {
        match edit {
            TreeEdit::InsertFraction { parent, index } => {
                self.editor
                    .tree_mut()
                    .get_mut_sequence(parent)
                    .unwrap()
                    .insert_container(index, ContainerKind::fraction)
                    .expect("Failed to create child");
            }
            TreeEdit::AddFraction { parent } => {
                self.editor
                    .tree_mut()
                    .get_mut_sequence(parent)
                    .unwrap()
                    .add_container(ContainerKind::fraction)
                    .expect("Failed to create child");
            }
            TreeEdit::DeleteContainer { id } => {
                self.editor
                    .tree_mut()
                    .get_mut_container(id)
                    .unwrap()
                    .delete();
            }
            TreeEdit::AddAtom { atom, parent } => self
                .editor
                .tree_mut()
                .get_mut_sequence(parent)
                .unwrap()
                .add_atom(atom),
            TreeEdit::InsertAtom {
                parent,
                index,
                atom,
            } => self
                .editor
                .tree_mut()
                .get_mut_sequence(parent)
                .unwrap()
                .insert_atom(index, atom),
            TreeEdit::ReplaceAtom {
                parent,
                index,
                atom,
            } => {
                match self
                    .editor
                    .tree_mut()
                    .get_mut_sequence(parent)
                    .unwrap()
                    .get_child_mut(index)
                    .unwrap()
                {
                    matatree::tree::ChildMut::Atom(a) => *a = atom,
                    matatree::tree::ChildMut::Container(_) => {
                        log::error!("Tried to replace an atom but found a container")
                    }
                };
            }
            TreeEdit::DeleteChild { parent, index } => assert!(
                self.editor
                    .tree_mut()
                    .get_mut_sequence(parent)
                    .unwrap()
                    .delete_child(index)
                    .is_some()
            ),
            TreeEdit::Left => self.editor.move_left(),
            TreeEdit::Right => self.editor.move_right(),
        }
    }
}
