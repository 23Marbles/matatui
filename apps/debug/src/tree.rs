use std::collections::HashMap;

use egui::{CollapsingHeader, WidgetText};
use matatree::{
    editor::Editor,
    tree::{Atom, ChildRef, ContainerRef, SequenceId, SequenceRef},
};

use crate::{app, command::TreeEdit};

pub fn draw_tree(context: &mut app::Context<'_>) {
    let app::Context {
        app,
        ui,
        frame,
        edit_list,
    } = context;

    debug_sequence(
        app.editor.tree().root(),
        "Root",
        ui,
        frame,
        edit_list,
        &mut app.atom_buffers,
    );
}

pub fn debug_sequence(
    sequence: SequenceRef,
    label: impl Into<WidgetText>,
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    edits: &mut Vec<TreeEdit>,
    atom_buffers: &mut HashMap<(SequenceId, usize), String>,
) {
    ui.horizontal_top(|ui| {
        CollapsingHeader::new(label)
            .id_salt(sequence.id())
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    for (index, child) in sequence.children().into_iter().enumerate() {
                        ui.horizontal(|ui| match child {
                            ChildRef::Atom(atom) => {
                                ui.push_id(index, |ui| match atom {
                                    Atom::Text(text) => {
                                        let key = (sequence.id(), index);

                                        let value =
                                            atom_buffers.entry(key).or_insert_with(|| text.clone());

                                        if ui.text_edit_singleline(value).changed() {
                                            edits.push(TreeEdit::ReplaceAtom {
                                                parent: sequence.id(),
                                                index,
                                                atom: Atom::Text(value.clone()),
                                            });
                                        }

                                        if ui.small_button("×").clicked() {
                                            edits.push(TreeEdit::DeleteChild {
                                                index,
                                                parent: sequence.id(),
                                            });
                                        }
                                    }

                                    Atom::Operator(ch) => {
                                        let key = (sequence.id(), index);

                                        let value = atom_buffers
                                            .entry(key)
                                            .or_insert_with(|| ch.to_string());

                                        let first_char_len = value.ceil_char_boundary(1);

                                        value.truncate(first_char_len);

                                        if ui.text_edit_singleline(value).changed() {
                                            if let Some(ch) = value.chars().next() {
                                                edits.push(TreeEdit::ReplaceAtom {
                                                    parent: sequence.id(),
                                                    index,
                                                    atom: Atom::Operator(ch),
                                                });
                                            }
                                        }

                                        if ui.small_button("×").clicked() {
                                            edits.push(TreeEdit::DeleteChild {
                                                index,
                                                parent: sequence.id(),
                                            });
                                        }
                                    }
                                });
                            }

                            ChildRef::Container(container) => {
                                debug_container(container, ui, frame, edits, atom_buffers);
                            }
                        });
                    }
                });
            });

        if ui.small_button("+").clicked() {
            edits.push(TreeEdit::AddAtom {
                atom: Atom::Operator('+'),
                parent: sequence.id(),
            });
        };

        if ui.small_button("𝑥").clicked() {
            edits.push(TreeEdit::AddAtom {
                atom: Atom::Text("x".to_string()),
                parent: sequence.id(),
            });
        };

        if ui.small_button("/").clicked() {
            edits.push(TreeEdit::AddFraction {
                parent: sequence.id(),
            });
        };
    });
}

pub fn debug_container(
    container: ContainerRef,
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    edits: &mut Vec<TreeEdit>,
    atom_buffers: &mut HashMap<(SequenceId, usize), String>,
) {
    let (name, children): (&'static str, Vec<(String, SequenceRef)>) = match container.kind() {
        matatree::tree::ContainerKind::Fraction {
            numerator,
            denominator,
        } => (
            "Fraction",
            vec![
                (
                    "Numerator".to_string(),
                    container.tree().get_sequence(*numerator).unwrap(),
                ),
                (
                    "Denominator".to_string(),
                    container.tree().get_sequence(*denominator).unwrap(),
                ),
            ],
        ),
    };

    ui.horizontal_top(|ui| {
        CollapsingHeader::new(name)
            .id_salt(container.id())
            .show(ui, |ui| {
                ui.indent(("container-content", container.id()), |ui| {
                    for (label, sequence) in children {
                        debug_sequence(sequence, label, ui, frame, edits, atom_buffers);
                    }
                });
            });

        if ui.small_button("×").clicked() {
            edits.push(TreeEdit::DeleteContainer { id: container.id() });
        }
    });
}
