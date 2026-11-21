use matatree::{
    editor::cursor::{Cursor, CursorControl},
    tree::{Atom, ChildRef, ContainerKind, SequenceId, Tree},
};

use super::*;

fn tree_with_script(
    kind: impl FnOnce([SequenceId; 1]) -> ContainerKind,
) -> (Tree, ContainerKind, SequenceId) {
    let mut tree = Tree::new();
    let container_id = tree
        .get_mut_sequence(tree.root_id())
        .unwrap()
        .add_container(kind)
        .unwrap();
    let kind = tree.get_container(container_id).unwrap().kind().clone();
    let sequence = match &kind {
        ContainerKind::SubScript { sequence } | ContainerKind::SuperScript { sequence } => *sequence,
        _ => panic!("Expected script"),
    };
    tree.get_mut_sequence(sequence)
        .unwrap()
        .add_atom(Atom::Text("2".into()));
    (tree, kind, sequence)
}

#[test]
fn cursor_after_fraction_tracks_the_following_atom() {
    let mut tree = Tree::new();
    let fraction_id = {
        let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
        root.add_atom(Atom::Text("a".into()));
        let fraction_id = root.add_container(ContainerKind::fraction).unwrap();
        root.add_atom(Atom::Text("b".into()));
        fraction_id
    };
    let (numerator, denominator) = match tree.get_container(fraction_id).unwrap().kind() {
        ContainerKind::Fraction {
            numerator,
            denominator,
        } => (*numerator, *denominator),
        _ => {
            panic!("Expected fraction")
        }
    };

    let root = tree.root();
    let cursor = root.move_right(Cursor::new(&tree)).unwrap();
    let cursor = root.move_right(cursor).unwrap();
    let cursor = tree
        .get_sequence(numerator)
        .unwrap()
        .move_right(cursor)
        .unwrap();
    let cursor = tree
        .get_sequence(denominator)
        .unwrap()
        .move_right(cursor)
        .unwrap();
    let cursor = root.move_right(cursor).unwrap();

    let layout = RatatuiLayout::layout_sequence(root, Some(&cursor));
    assert_eq!(layout.cursor_pos, Some((7, 1, 1)));
}

#[test]
fn subscript_layout_occupies_the_region_below_the_baseline() {
    let (tree, kind, sequence) =
        tree_with_script(|[sequence]| ContainerKind::sub_script([sequence]));
    let script = tree.get_container(
        tree.root()
            .children()
            .into_iter()
            .find_map(|child| match child {
                ChildRef::Container(container) => Some(container.id()),
                ChildRef::Atom(_) => None,
            })
            .unwrap(),
    );

    let layout = RatatuiLayout::layout_sequence(tree.root(), None);
    assert_eq!(layout.measurement.ascent, 1);
    assert_eq!(layout.measurement.descent, 1);

    let script_layout = RatatuiLayout::layout_container(script.unwrap(), None);
    assert_eq!(script_layout.measurement.ascent, 0);
    assert_eq!(script_layout.measurement.descent, 1);
    assert!(matches!(kind, ContainerKind::SubScript { sequence: id } if id == sequence));
}

#[test]
fn superscript_layout_occupies_the_region_above_the_baseline() {
    let (tree, kind, sequence) =
        tree_with_script(|[sequence]| ContainerKind::super_script([sequence]));
    let script = tree.get_container(
        tree.root()
            .children()
            .into_iter()
            .find_map(|child| match child {
                ChildRef::Container(container) => Some(container.id()),
                ChildRef::Atom(_) => None,
            })
            .unwrap(),
    );

    let layout = RatatuiLayout::layout_sequence(tree.root(), None);
    assert_eq!(layout.measurement.ascent, 1);
    assert_eq!(layout.measurement.descent, 0);

    let script_layout = RatatuiLayout::layout_container(script.unwrap(), None);
    assert_eq!(script_layout.measurement.ascent, 1);
    assert_eq!(script_layout.measurement.descent, 0);
    assert!(matches!(kind, ContainerKind::SuperScript { sequence: id } if id == sequence));
}
