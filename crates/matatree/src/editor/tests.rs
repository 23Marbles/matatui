use super::*;
use crate::tree::{Atom, ContainerId, ContainerKind, SequenceId};

fn tree_with_fraction() -> (Tree, ContainerId, SequenceId, SequenceId) {
    let mut tree = Tree::new();
    let container_id = tree
        .get_mut_sequence(tree.root_id())
        .unwrap()
        .add_container(|[numerator, denominator]| ContainerKind::fraction([numerator, denominator]))
        .unwrap();
    let (numerator, denominator) = match tree.get_container(container_id).unwrap().kind() {
        ContainerKind::Fraction {
            numerator,
            denominator,
        } => (*numerator, *denominator),
    };
    tree.get_mut_sequence(numerator)
        .unwrap()
        .add_atom(Atom::Text("N".into()));
    tree.get_mut_sequence(denominator)
        .unwrap()
        .add_atom(Atom::Text("D".into()));
    (tree, container_id, numerator, denominator)
}

#[test]
fn from_tree_starts_cursor_at_root_and_exposes_cursor_sequence() {
    let editor = CursoredTree::from_tree(Tree::new());

    assert_eq!(editor.cursor().sequence_id(), editor.tree().root_id());
    assert_eq!(editor.cursor().child_index(), None);
    assert_eq!(editor.sequence_at_cursor().id(), editor.tree().root_id());
}

#[test]
fn default_editor_has_an_empty_root_sequence() {
    let editor = CursoredTree::default();

    assert_eq!(editor.cursor().sequence_id(), editor.tree().root_id());
    assert_eq!(editor.tree().root().child_count(), 0);
}

#[test]
fn editor_navigation_enters_and_exits_fraction_sequences() {
    let (tree, _, numerator, denominator) = tree_with_fraction();
    let mut editor = CursoredTree::from_tree(tree);

    editor.move_right();
    assert_eq!(editor.cursor().sequence_id(), numerator);
    assert_eq!(editor.cursor().child_index(), None);

    editor.move_right();
    assert_eq!(editor.cursor().sequence_id(), numerator);
    assert_eq!(editor.cursor().child_index(), Some((0, Some(0))));

    editor.move_right();
    assert_eq!(editor.cursor().sequence_id(), denominator);
    assert_eq!(editor.cursor().child_index(), None);

    editor.move_right();
    assert_eq!(editor.cursor().sequence_id(), denominator);
    assert_eq!(editor.cursor().child_index(), Some((0, Some(0))));

    editor.move_right();
    assert_eq!(editor.cursor().sequence_id(), editor.tree().root_id());
    assert_eq!(editor.cursor().child_index(), Some((0, None)));

    editor.move_left();
    assert_eq!(editor.cursor().sequence_id(), denominator);
    assert_eq!(editor.cursor().child_index(), Some((0, Some(0))));

    editor.move_left();
    assert_eq!(editor.cursor().sequence_id(), denominator);
    assert_eq!(editor.cursor().child_index(), None);

    editor.move_left();
    assert_eq!(editor.cursor().sequence_id(), numerator);
    assert_eq!(editor.cursor().child_index(), Some((0, Some(0))));
}

#[test]
fn backspace_at_fraction_start_flattens_it_and_inside_backspace_edits_text() {
    let (tree, container_id, numerator, denominator) = tree_with_fraction();
    let mut editor = CursoredTree::from_tree(tree);

    editor.move_right();
    editor.handle_event(event::Event::Backspace);

    assert!(editor.tree().get_container(container_id).is_none());
    assert!(editor.tree().get_sequence(numerator).is_none());
    assert!(editor.tree().get_sequence(denominator).is_none());
    assert_eq!(editor.cursor().sequence_id(), editor.tree().root_id());
    assert!(matches!(
        editor.tree().root().get_child(0),
        Some(crate::tree::ChildRef::Atom(Atom::Text(text))) if text == "ND"
    ));

    let (tree, container_id, numerator, _) = tree_with_fraction();
    let mut editor = CursoredTree::from_tree(tree);
    editor.move_right();
    editor.move_right();
    editor.handle_event(event::Event::Backspace);

    assert!(editor.tree().get_container(container_id).is_some());
    assert_eq!(
        editor.tree().get_sequence(numerator).unwrap().child_count(),
        0
    );
}

#[test]
fn backspace_at_denominator_start_flattens_fraction_and_restores_cursor() {
    let (tree, container_id, _, denominator) = tree_with_fraction();
    let mut editor = CursoredTree::from_tree(tree);
    editor.move_right();
    editor.move_right();
    editor.move_right();

    assert_eq!(editor.cursor().sequence_id(), denominator);
    assert_eq!(editor.cursor().child_index(), None);
    editor.handle_event(event::Event::Backspace);

    assert!(editor.tree().get_container(container_id).is_none());
    assert!(matches!(
        editor.tree().root().get_child(0),
        Some(crate::tree::ChildRef::Atom(Atom::Text(text))) if text == "ND"
    ));
    assert_eq!(editor.cursor().sequence_id(), editor.tree().root_id());
    assert_eq!(editor.cursor().child_index(), Some((0, Some(0))));
}

#[test]
fn backspace_at_empty_fraction_start_removes_the_fraction() {
    let mut tree = Tree::new();
    let container_id = tree
        .get_mut_sequence(tree.root_id())
        .unwrap()
        .add_container(ContainerKind::fraction)
        .unwrap();
    let (numerator, denominator) = match tree.get_container(container_id).unwrap().kind() {
        ContainerKind::Fraction {
            numerator,
            denominator,
        } => (*numerator, *denominator),
    };
    let mut editor = CursoredTree::from_tree(tree);
    editor.move_right();
    editor.handle_event(event::Event::Backspace);

    assert!(editor.tree().get_container(container_id).is_none());
    assert!(editor.tree().get_sequence(numerator).is_none());
    assert!(editor.tree().get_sequence(denominator).is_none());
    assert_eq!(editor.tree().root().child_count(), 0);
    assert_eq!(editor.cursor().sequence_id(), editor.tree().root_id());
    assert_eq!(editor.cursor().child_index(), None);
}
