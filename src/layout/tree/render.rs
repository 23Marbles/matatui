use ratatui::{buffer::Buffer, layout::Rect};

use crate::layout::tree::{ContainerKind, container::ContainerRef, sequence::SequenceRef};

pub struct Measurement {
    pub width: u16,
    pub ascent: u16,
    pub descent: u16,
}

impl Measurement {
    pub fn height(&self) -> u16 {
        self.ascent + 1 + self.descent
    }
}

pub struct LayoutNode {
    pub measurement: Measurement,
    pub kind: LayoutKind,
}

pub enum LayoutKind {
    Sequence {
        children: Vec<PositionedChild>,
    },
    Fraction {
        numerator: Box<LayoutNode>,
        denominator: Box<LayoutNode>,
    },
    Text(String),
    Operator(char),
    // Matrix, square root, etc.
}

pub struct PositionedChild {
    pub x: u16,
    pub baseline_y: u16,
    pub node: Box<LayoutNode>,
}

pub fn layout_sequence(seq: SequenceRef<'_>) -> LayoutNode {
    // 1. Iterate over seq.children().
    // 2. Measure text and operators.
    // 3. Resolve container IDs through the tree.
    // 4. Recursively lay out each container.
    // 5. Calculate the sequence's ascent, descent and width.
    // 6. Assign each child's relative position.
    todo!()
}

fn layout_container(container: ContainerRef<'_>) -> LayoutNode {
    match container.kind() {
        ContainerKind::Fraction {
            numerator,
            denominator,
        } => {
            // Resolve both SequenceIds through the tree.
            // Layout both sequences.
            // Centre them horizontally.
            // Calculate the fraction's baseline.
            todo!()
        } // ContainerKind::Matrix { ... } => { ... }
          // ContainerKind::SquareRoot { ... } => { ... }
    }
}

pub fn draw_node(node: &LayoutNode, area: Rect, buf: &mut Buffer) {
    match &node.kind {
        LayoutKind::Sequence { children } => {
            for child in children {
                let child_area = Rect {
                    x: area.x + child.x,
                    y: area.y + child.baseline_y - child.node.measurement.ascent,
                    width: child.node.measurement.width,
                    height: child.node.measurement.height(),
                };

                draw_node(&child.node, child_area, buf);
            }
        }

        LayoutKind::Fraction {
            numerator,
            denominator,
        } => {
            // Draw numerator and denominator in their
            // precomputed positions, then draw the bar.
        }

        LayoutKind::Text(text) => {
            // Render text within area.
        }

        LayoutKind::Operator(operator) => {
            // Render operator within area.
        }
    }
}
