use std::collections::HashMap;

use egui::CollapsingHeader;
use matatree::tree::{ChildRef, ContainerRef, SequenceId, SequenceRef};

use crate::{app::ContextEdit, command::TreeEdit};

use super::debug_sequence;

pub fn debug_container(
    container: ContainerRef,
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    edit: &mut ContextEdit,
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
        matatree::tree::ContainerKind::SubScript { sequence } => (
            "Sub Script",
            vec![(
                "Sequence".to_string(),
                container.tree().get_sequence(*sequence).unwrap(),
            )],
        ),
        matatree::tree::ContainerKind::SuperScript { sequence } => (
            "Super Script",
            vec![(
                "Sequence".to_string(),
                container.tree().get_sequence(*sequence).unwrap(),
            )],
        ),
    };

    let mut header_edits = Vec::new();
    let mut remove_container = false;

    let edit_is_mut = edit.is_mut();

    CollapsingHeader::custom(
        ui,
        container.id(),
        |ui| {
            ui.label(name);
            if edit_is_mut {
                if ui.small_button("×").clicked() {
                    remove_container = true;
                    header_edits.push(TreeEdit::DeleteContainer { id: container.id() });
                }
            }
        },
        |ui| {
            for (label, sequence) in children {
                debug_sequence(sequence, label, ui, frame, edit);
            }
        },
    );

    if remove_container && let ContextEdit::Editable { atom_buffers, .. } = edit {
        remove_container_atom_buffers(container, atom_buffers);
    }

    if header_edits.is_empty() {
        return;
    }

    let ContextEdit::Editable { edit_list, .. } = edit else {
        unreachable!("Context edit should not allow edits to be input")
    };

    edit_list.extend(header_edits);
}

fn remove_container_atom_buffers(
    container: ContainerRef,
    atom_buffers: &mut HashMap<(SequenceId, usize), String>,
) {
    match container.kind() {
        matatree::tree::ContainerKind::Fraction {
            numerator,
            denominator,
        } => {
            remove_sequence_atom_buffers(
                container.tree().get_sequence(*numerator).unwrap(),
                atom_buffers,
            );
            remove_sequence_atom_buffers(
                container.tree().get_sequence(*denominator).unwrap(),
                atom_buffers,
            );
        }
        matatree::tree::ContainerKind::SubScript { sequence }
        | matatree::tree::ContainerKind::SuperScript { sequence } => remove_sequence_atom_buffers(
            container.tree().get_sequence(*sequence).unwrap(),
            atom_buffers,
        ),
    }
}

fn remove_sequence_atom_buffers(
    sequence: SequenceRef,
    atom_buffers: &mut HashMap<(SequenceId, usize), String>,
) {
    let id = sequence.id();

    for (idx, child) in sequence.children().into_iter().enumerate() {
        match child {
            ChildRef::Atom(_) => {
                atom_buffers.remove(&(id, idx));
            }
            ChildRef::Container(container) => {
                remove_container_atom_buffers(container, atom_buffers)
            }
        }
    }
}
