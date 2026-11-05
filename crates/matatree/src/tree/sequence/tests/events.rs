use crate::{
    editor::{cursor::Cursor, event},
    tree::{Atom, ChildRef, ContainerKind, Tree},
};

#[test]
fn backspace_deletes_operator_and_joins_neighbors() {
    let mut tree = Tree::new();
    let mut cursor;
    {
        let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
        root.add_atom(Atom::Text("ab".into()));
        root.add_atom(Atom::Operator('+'));
        root.add_atom(Atom::Text("cd".into()));

        cursor = Cursor::new_raw(root.id(), Some((1, None)));
        root.handle_event(event::Event::Backspace, &mut cursor);
    }

    assert!(matches!(
        tree.root().get_child(0),
        Some(ChildRef::Atom(Atom::Text(text))) if text == "abcd"
    ));
    assert_eq!(tree.root().child_count(), 1);
    assert_eq!(cursor.child_index(), Some((0, Some(1))));
}

#[test]
fn backspace_removes_an_entire_unicode_grapheme() {
    let mut tree = Tree::new();
    let root_id = tree.root_id();
    tree.get_mut_sequence(root_id)
        .unwrap()
        .add_atom(Atom::Text("a👩‍💻z".into()));
    let mut cursor = Cursor::new_raw(root_id, Some((0, Some(1))));

    tree.get_mut_sequence(root_id)
        .unwrap()
        .handle_event(event::Event::Backspace, &mut cursor);

    assert!(matches!(
        tree.root().get_child(0),
        Some(ChildRef::Atom(Atom::Text(text))) if text == "az"
    ));
    assert_eq!(cursor.child_index(), Some((0, Some(0))));
}

#[test]
fn moving_children_updates_nested_container_parent() {
    let mut tree = Tree::new();
    let (first, second, first_numerator) = {
        let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
        let first = root
            .add_container(|[n, d]| ContainerKind::fraction([n, d]))
            .unwrap();
        let second = root
            .add_container(|[n, d]| ContainerKind::fraction([n, d]))
            .unwrap();
        let ContainerKind::Fraction { numerator, .. } =
            root.tree().get_container(first).unwrap().kind().clone();
        (first, second, numerator)
    };
    let second_numerator = match tree.get_container(second).unwrap().kind() {
        ContainerKind::Fraction { numerator, .. } => *numerator,
    };
    assert_eq!(
        tree.get_container(first).unwrap().parent_id(),
        tree.root_id()
    );
    let child_container = tree
        .get_mut_sequence(first_numerator)
        .unwrap()
        .add_container(|[n, d]| ContainerKind::fraction([n, d]))
        .unwrap();

    tree.move_children(first_numerator, 0..1, second_numerator, 0);

    assert_eq!(
        tree.get_container(child_container).unwrap().parent_id(),
        second_numerator
    );
    assert_eq!(tree.get_sequence(first_numerator).unwrap().child_count(), 0);
    assert_eq!(
        tree.get_sequence(second_numerator).unwrap().child_count(),
        1
    );
}
