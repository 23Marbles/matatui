use std::ops::DerefMut;

use matatree::{
    editor::CursoredTree,
    tree::{Atom, ContainerId, ContainerKind, SequenceId},
};

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
        let apply_edit = |tree: &mut CursoredTree| match edit {
            TreeEdit::InsertFraction { parent, index } => {
                tree.tree_mut()
                    .get_mut_sequence(parent)
                    .unwrap()
                    .insert_container(index, ContainerKind::fraction)
                    .expect("Failed to create child");
            }
            TreeEdit::AddFraction { parent } => {
                tree.tree_mut()
                    .get_mut_sequence(parent)
                    .unwrap()
                    .add_container(ContainerKind::fraction)
                    .expect("Failed to create child");
            }
            TreeEdit::DeleteContainer { id } => {
                tree.tree_mut().get_mut_container(id).unwrap().delete();
            }
            TreeEdit::AddAtom { atom, parent } => tree
                .tree_mut()
                .get_mut_sequence(parent)
                .unwrap()
                .add_atom(atom),
            TreeEdit::InsertAtom {
                parent,
                index,
                atom,
            } => tree
                .tree_mut()
                .get_mut_sequence(parent)
                .unwrap()
                .insert_atom(index, atom),
            TreeEdit::ReplaceAtom {
                parent,
                index,
                atom,
            } => {
                match tree
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
                tree.tree_mut()
                    .get_mut_sequence(parent)
                    .unwrap()
                    .delete_child(index)
                    .is_some()
            ),
            TreeEdit::Left => tree.move_left(),
            TreeEdit::Right => tree.move_right(),
        };

        match self.kind {
            crate::app::AppKind::ImmutableMirror { .. }
            | crate::app::AppKind::ImmutableOwned { .. } => {
                log::warn!("Tried to apply edit to an immutable `DebugApp`")
            }
            crate::app::AppKind::MutableMirror {
                shared_tree: ref tree,
                ref mut cache,
                ..
            } => {
                let mut tree = tree.write();
                apply_edit(tree.deref_mut());
                *cache = tree.clone();
            }
            crate::app::AppKind::Owned { ref mut editor, .. } => apply_edit(editor),
        }
    }
}
