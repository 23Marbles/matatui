use matatree::{
    editor::cursor::{Cursor, CursorControl},
    tree::{Atom, ContainerKind, Tree},
};

use super::*;

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
    assert_eq!(layout.cursor_pos, Some((5, 1, 1)));
}
