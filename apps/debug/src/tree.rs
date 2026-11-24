use egui::{CollapsingHeader, WidgetText};
use matatree::{
    editor::Editor,
    tree::{Atom, ChildRef, OperatorKind, SequenceRef},
};

use crate::{
    app::{self, ContextEdit},
    command::TreeEdit,
};

mod container;
pub use container::debug_container;

pub fn draw_tree(context: &mut app::Context<'_>) {
    let app::Context {
        ui,
        frame,
        tree,
        edit,
    } = context;

    debug_sequence(tree.tree().root(), "Root", ui, frame, edit);
}

pub fn debug_sequence(
    sequence: SequenceRef,
    label: impl Into<WidgetText>,
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    edit: &mut ContextEdit<'_>,
) {
    let mut header_edits = Vec::new();

    let edit_is_mut = edit.is_mut();

    CollapsingHeader::custom(
        ui,
        sequence.id(),
        |ui| {
            ui.label(label);
            if edit_is_mut {
                if ui.small_button("+").clicked() {
                    header_edits.push(TreeEdit::AddAtom {
                        atom: Atom::Operator(OperatorKind::Add),
                        parent: sequence.id(),
                    });
                }

                if ui.small_button("𝑥").clicked() {
                    header_edits.push(TreeEdit::AddAtom {
                        atom: Atom::Text("x".to_string()),
                        parent: sequence.id(),
                    });
                }

                if ui.small_button("/").clicked() {
                    header_edits.push(TreeEdit::AddFraction {
                        parent: sequence.id(),
                    });
                }
            }
        },
        |ui| {
            ui.vertical(|ui| {
                for (index, child) in sequence.children().into_iter().enumerate() {
                    match child {
                        ChildRef::Atom(atom) => {
                            ui.horizontal_top(|ui| {
                                let edit_width = ui.available_width()
                                    - ui.spacing().interact_size.x
                                    - ui.spacing().item_spacing.x;

                                ui.push_id(index, |ui| match atom {
                                    Atom::Text(text) => match edit {
                                        ContextEdit::Editable {
                                            edit_list,
                                            atom_buffers,
                                        } => {
                                            let key = (sequence.id(), index);

                                            let value = atom_buffers
                                                .entry(key)
                                                .or_insert_with(|| text.clone());

                                            if ui
                                                .add(
                                                    egui::TextEdit::singleline(value)
                                                        .desired_width(edit_width),
                                                )
                                                .changed()
                                            {
                                                edit_list.push(TreeEdit::ReplaceAtom {
                                                    parent: sequence.id(),
                                                    index,
                                                    atom: Atom::Text(value.clone()),
                                                });
                                            }

                                            if ui.small_button("×").clicked() {
                                                atom_buffers.remove(&key);

                                                edit_list.push(TreeEdit::DeleteChild {
                                                    index,
                                                    parent: sequence.id(),
                                                });
                                            }
                                        }
                                        ContextEdit::Immutable => {
                                            ui.label(text);
                                        }
                                    },
                                    Atom::Operator(kind) => match edit {
                                        ContextEdit::Editable {
                                            edit_list,
                                            atom_buffers,
                                        } => {
                                            let key = (sequence.id(), index);

                                            let value = atom_buffers
                                                .entry(key)
                                                .or_insert_with(|| kind.to_string());

                                            let first_char_len = value.ceil_char_boundary(1);

                                            value.truncate(first_char_len);

                                            if ui
                                                .add(
                                                    egui::TextEdit::singleline(value)
                                                        .char_limit(1)
                                                        .desired_width(edit_width),
                                                )
                                                .changed()
                                                && let Some(ch) = value.chars().next()
                                                && let Some(kind) = OperatorKind::from_char(ch)
                                            {
                                                edit_list.push(TreeEdit::ReplaceAtom {
                                                    parent: sequence.id(),
                                                    index,
                                                    atom: Atom::Operator(kind),
                                                });
                                            }

                                            if ui.small_button("×").clicked() {
                                                atom_buffers.remove(&key);

                                                edit_list.push(TreeEdit::DeleteChild {
                                                    index,
                                                    parent: sequence.id(),
                                                });
                                            }
                                        }
                                        ContextEdit::Immutable => {
                                            ui.label(kind.to_char().to_string());
                                        }
                                    },
                                });
                            });
                        }

                        ChildRef::Container(container) => {
                            debug_container(container, ui, frame, edit);
                        }
                    }
                }
            });
        },
    );

    if header_edits.is_empty() {
        return;
    }

    let ContextEdit::Editable { edit_list, .. } = edit else {
        unreachable!("Context edit should not allow edits to be input")
    };

    edit_list.extend(header_edits);
}
