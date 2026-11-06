use crate::tree::{Atom, Tree};

use super::*;

#[test]
fn rightmost_cursor_uses_grapheme_count_for_unicode() {
    let mut tree = Tree::new();
    let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
    root.add_atom(Atom::Text("é".into()));

    let cursor = tree.root().rightmost_cursor();

    assert_eq!(cursor.child_index, Some((0, Some(0))));
}

#[test]
fn moving_left_from_operator_lands_on_last_text_grapheme() {
    let mut tree = Tree::new();
    {
        let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
        root.add_atom(Atom::Text("xy".into()));
        root.add_atom(Atom::Operator(crate::tree::OperatorKind::Add));
        root.add_atom(Atom::Text("z".into()));
    }

    let root = tree.root();
    let cursor = root.move_right(Cursor::new(&tree)).unwrap();
    let cursor = root.move_right(cursor).unwrap();
    let cursor = root.move_right(cursor).unwrap();
    assert_eq!(cursor.child_index, Some((1, None)));

    let cursor = root.move_left(cursor).unwrap();
    assert_eq!(cursor.child_index, Some((0, Some(1))));

    let cursor = root.move_right(cursor).unwrap();
    assert_eq!(cursor.child_index, Some((1, None)));
    let cursor = root.move_right(cursor).unwrap();
    assert_eq!(cursor.child_index, Some((2, Some(0))));
}

#[test]
fn movement_steps_through_unicode_graphemes_and_stops_at_root_edges() {
    let mut tree = Tree::new();
    let root_id = tree.root_id();
    tree.get_mut_sequence(root_id)
        .unwrap()
        .add_atom(Atom::Text("a👩‍💻b".into()));

    let root = tree.root();
    let start = Cursor::new(&tree);
    assert!(root.move_left(start).is_none());

    let first = root.move_right(start).unwrap();
    let second = root.move_right(first).unwrap();
    let third = root.move_right(second).unwrap();
    assert_eq!(first.child_index(), Some((0, Some(0))));
    assert_eq!(second.child_index(), Some((0, Some(1))));
    assert_eq!(third.child_index(), Some((0, Some(2))));
    assert!(root.move_right(third).is_none());
    assert_eq!(
        root.move_left(third).unwrap().child_index(),
        Some((0, Some(1)))
    );
}
