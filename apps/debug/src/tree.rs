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
    let mut header_edits = Vec::new();
    let mut remove_container = false;

    CollapsingHeader::custom(
        ui,
        sequence.id(),
        |ui| {
            ui.label(label);
            if ui.small_button("+").clicked() {
                header_edits.push(TreeEdit::AddAtom {
                    atom: Atom::Operator('+'),
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
                                    Atom::Text(text) => {
                                        let key = (sequence.id(), index);

                                        let value =
                                            atom_buffers.entry(key).or_insert_with(|| text.clone());

                                        if ui
                                            .add(
                                                egui::TextEdit::singleline(value)
                                                    .desired_width(edit_width),
                                            )
                                            .changed()
                                        {
                                            edits.push(TreeEdit::ReplaceAtom {
                                                parent: sequence.id(),
                                                index,
                                                atom: Atom::Text(value.clone()),
                                            });
                                        }

                                        if ui.small_button("×").clicked() {
                                            atom_buffers.remove(&key);

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

                                        if ui
                                            .add(
                                                egui::TextEdit::singleline(value)
                                                    .desired_width(edit_width),
                                            )
                                            .changed()
                                        {
                                            if let Some(ch) = value.chars().next() {
                                                edits.push(TreeEdit::ReplaceAtom {
                                                    parent: sequence.id(),
                                                    index,
                                                    atom: Atom::Operator(ch),
                                                });
                                            }
                                        }

                                        if ui.small_button("×").clicked() {
                                            atom_buffers.remove(&key);

                                            edits.push(TreeEdit::DeleteChild {
                                                index,
                                                parent: sequence.id(),
                                            });
                                        }
                                    }
                                });
                            });
                        }

                        ChildRef::Container(container) => {
                            debug_container(container, ui, frame, edits, atom_buffers);
                        }
                    }
                }
            });
        },
    );

    edits.extend(header_edits);
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

    let mut header_edits = Vec::new();

    let mut remove_container = false;

    CollapsingHeader::custom(
        ui,
        container.id(),
        |ui| {
            ui.label(name);
            if ui.small_button("×").clicked() {
                remove_container = true;
                header_edits.push(TreeEdit::DeleteContainer { id: container.id() });
            }
        },
        |ui| {
            for (label, sequence) in children {
                debug_sequence(sequence, label, ui, frame, edits, atom_buffers);
            }
        },
    );

    if remove_container {
        fn remove_atom_buffers(
            sequence: SequenceRef,
            atom_buffers: &mut HashMap<(SequenceId, usize), String>,
        ) {
            let id = sequence.id();

            for (idx, child) in sequence.children().into_iter().enumerate() {
                match child {
                    ChildRef::Atom(_) => {
                        atom_buffers.remove(&(id, idx));
                    }
                    ChildRef::Container(container_ref) => {
                        remove_containers_atom_buffers(container_ref, atom_buffers)
                    }
                }
            }
        }

        fn remove_containers_atom_buffers(
            container_ref: ContainerRef,
            atom_buffers: &mut HashMap<(SequenceId, usize), String>,
        ) {
            match container_ref.kind() {
                matatree::tree::ContainerKind::Fraction {
                    numerator,
                    denominator,
                } => {
                    remove_atom_buffers(
                        container_ref.tree().get_sequence(*numerator).unwrap(),
                        atom_buffers,
                    );
                    remove_atom_buffers(
                        container_ref.tree().get_sequence(*denominator).unwrap(),
                        atom_buffers,
                    );
                }
            }
        }

        remove_containers_atom_buffers(container, atom_buffers);
    }

    edits.extend(header_edits);
}
