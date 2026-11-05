use std::collections::HashMap;

use egui::CollapsingHeader;
use matatree::tree::{ChildRef, ContainerRef, SequenceId, SequenceRef};

use crate::command::TreeEdit;

use super::debug_sequence;

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
        remove_container_atom_buffers(container, atom_buffers);
    }

    edits.extend(header_edits);
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
