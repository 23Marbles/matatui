use crate::{
    editor::{cursor::Cursor, event},
    tree::{Atom, ChildRef, ContainerId, ContainerKind, SequenceId, Tree},
};

#[derive(Clone, Copy, Debug)]
enum CursorSide {
    Numerator,
    Denominator,
}

#[derive(Clone, Copy, Debug)]
enum NeighborContext {
    NoSiblings,
    TextBefore,
    TextAfter,
    TextOnBothSides,
    OperatorsOnBothSides,
    OperatorBeforeTextAfter,
    TextBeforeOperatorAfter,
}

impl NeighborContext {
    fn cases(self) -> (&'static str, Vec<Atom>, Vec<Atom>) {
        match self {
            Self::NoSiblings => ("no siblings", vec![], vec![]),
            Self::TextBefore => ("text before", vec![Atom::Text("L".into())], vec![]),
            Self::TextAfter => ("text after", vec![], vec![Atom::Text("R".into())]),
            Self::TextOnBothSides => (
                "text on both sides",
                vec![Atom::Text("L".into())],
                vec![Atom::Text("R".into())],
            ),
            Self::OperatorsOnBothSides => (
                "operators on both sides",
                vec![Atom::Operator(crate::tree::OperatorKind::Add)],
                vec![Atom::Operator(crate::tree::OperatorKind::Sub)],
            ),
            Self::OperatorBeforeTextAfter => (
                "operator before and text after",
                vec![Atom::Operator(crate::tree::OperatorKind::Add)],
                vec![Atom::Text("R".into())],
            ),
            Self::TextBeforeOperatorAfter => (
                "text before and operator after",
                vec![Atom::Text("L".into())],
                vec![Atom::Operator(crate::tree::OperatorKind::Sub)],
            ),
        }
    }
}

fn make_fraction(
    numerator_text: bool,
    denominator_text: bool,
    before: Vec<Atom>,
    after: Vec<Atom>,
) -> (Tree, ContainerId, SequenceId, SequenceId) {
    let mut tree = Tree::new();
    let root_id = tree.root_id();
    let container_id = {
        let mut root = tree.get_mut_sequence(root_id).unwrap();
        for atom in before {
            root.add_atom(atom);
        }
        let id = root
            .add_container(ContainerKind::fraction)
            .expect("fraction can be added to root");
        for atom in after {
            root.add_atom(atom);
        }
        id
    };
    let (numerator, denominator) = match tree.get_container(container_id).unwrap().kind() {
        ContainerKind::Fraction {
            numerator,
            denominator,
        } => (*numerator, *denominator),
    };
    if numerator_text {
        tree.get_mut_sequence(numerator)
            .unwrap()
            .add_atom(Atom::Text("N".into()));
    }
    if denominator_text {
        tree.get_mut_sequence(denominator)
            .unwrap()
            .add_atom(Atom::Text("D".into()));
    }

    (tree, container_id, numerator, denominator)
}

fn assert_backspace_at_fraction_start(
    side: CursorSide,
    context: NeighborContext,
    numerator_text: bool,
    denominator_text: bool,
) {
    let (context_name, before, after) = context.cases();
    let (mut tree, container_id, numerator, denominator) = make_fraction(
        numerator_text,
        denominator_text,
        before.clone(),
        after.clone(),
    );
    let root_id = tree.root_id();
    let sequence_id = match side {
        CursorSide::Numerator => numerator,
        CursorSide::Denominator => denominator,
    };
    let mut cursor = Cursor::new_raw(sequence_id, None);

    let result = tree
        .get_mut_container(container_id)
        .unwrap()
        .handle_event(event::Event::Backspace, &mut cursor);

    assert!(
        result.is_none(),
        "{side:?} start with {context_name} must flatten"
    );
    assert!(tree.get_container(container_id).is_none());
    assert!(tree.get_sequence(numerator).is_none());
    assert!(tree.get_sequence(denominator).is_none());

    let mut expected_raw = before;
    if numerator_text {
        expected_raw.push(Atom::Text("N".into()));
    }
    if denominator_text {
        expected_raw.push(Atom::Text("D".into()));
    }
    expected_raw.extend(after);
    let expected = coalesce_text(expected_raw);
    let actual: Vec<_> = tree
        .root()
        .children()
        .into_iter()
        .map(|child| match child {
            ChildRef::Atom(atom) => atom.clone(),
            ChildRef::Container(_) => {
                panic!("atom-only fixture unexpectedly produced a container")
            }
        })
        .collect();
    assert_eq!(actual, expected, "flattened child sequence");
    assert_eq!(cursor.sequence_id(), root_id);
    assert_cursor_is_valid(&tree, cursor);
}

macro_rules! fraction_start_test {
    ($name:ident, $side:expr, $context:expr, $numerator_text:expr, $denominator_text:expr) => {
        #[test]
        fn $name() {
            assert_backspace_at_fraction_start($side, $context, $numerator_text, $denominator_text);
        }
    };
}

fraction_start_test!(
    numerator_empty_empty_no_siblings,
    CursorSide::Numerator,
    NeighborContext::NoSiblings,
    false,
    false
);
fraction_start_test!(
    numerator_empty_empty_text_before,
    CursorSide::Numerator,
    NeighborContext::TextBefore,
    false,
    false
);
fraction_start_test!(
    numerator_empty_empty_text_after,
    CursorSide::Numerator,
    NeighborContext::TextAfter,
    false,
    false
);
fraction_start_test!(
    numerator_empty_empty_text_both_sides,
    CursorSide::Numerator,
    NeighborContext::TextOnBothSides,
    false,
    false
);
fraction_start_test!(
    numerator_empty_empty_operators_both_sides,
    CursorSide::Numerator,
    NeighborContext::OperatorsOnBothSides,
    false,
    false
);
fraction_start_test!(
    numerator_empty_empty_operator_before_text_after,
    CursorSide::Numerator,
    NeighborContext::OperatorBeforeTextAfter,
    false,
    false
);
fraction_start_test!(
    numerator_empty_empty_text_before_operator_after,
    CursorSide::Numerator,
    NeighborContext::TextBeforeOperatorAfter,
    false,
    false
);
fraction_start_test!(
    numerator_empty_denominator_no_siblings,
    CursorSide::Numerator,
    NeighborContext::NoSiblings,
    false,
    true
);
fraction_start_test!(
    numerator_empty_denominator_text_before,
    CursorSide::Numerator,
    NeighborContext::TextBefore,
    false,
    true
);
fraction_start_test!(
    numerator_empty_denominator_text_after,
    CursorSide::Numerator,
    NeighborContext::TextAfter,
    false,
    true
);
fraction_start_test!(
    numerator_empty_denominator_text_both_sides,
    CursorSide::Numerator,
    NeighborContext::TextOnBothSides,
    false,
    true
);
fraction_start_test!(
    numerator_empty_denominator_operators_both_sides,
    CursorSide::Numerator,
    NeighborContext::OperatorsOnBothSides,
    false,
    true
);
fraction_start_test!(
    numerator_empty_denominator_operator_before_text_after,
    CursorSide::Numerator,
    NeighborContext::OperatorBeforeTextAfter,
    false,
    true
);
fraction_start_test!(
    numerator_empty_denominator_text_before_operator_after,
    CursorSide::Numerator,
    NeighborContext::TextBeforeOperatorAfter,
    false,
    true
);
fraction_start_test!(
    numerator_present_empty_no_siblings,
    CursorSide::Numerator,
    NeighborContext::NoSiblings,
    true,
    false
);
fraction_start_test!(
    numerator_present_empty_text_before,
    CursorSide::Numerator,
    NeighborContext::TextBefore,
    true,
    false
);
fraction_start_test!(
    numerator_present_empty_text_after,
    CursorSide::Numerator,
    NeighborContext::TextAfter,
    true,
    false
);
fraction_start_test!(
    numerator_present_empty_text_both_sides,
    CursorSide::Numerator,
    NeighborContext::TextOnBothSides,
    true,
    false
);
fraction_start_test!(
    numerator_present_empty_operators_both_sides,
    CursorSide::Numerator,
    NeighborContext::OperatorsOnBothSides,
    true,
    false
);
fraction_start_test!(
    numerator_present_empty_operator_before_text_after,
    CursorSide::Numerator,
    NeighborContext::OperatorBeforeTextAfter,
    true,
    false
);
fraction_start_test!(
    numerator_present_empty_text_before_operator_after,
    CursorSide::Numerator,
    NeighborContext::TextBeforeOperatorAfter,
    true,
    false
);
fraction_start_test!(
    numerator_present_denominator_present_no_siblings,
    CursorSide::Numerator,
    NeighborContext::NoSiblings,
    true,
    true
);
fraction_start_test!(
    numerator_present_denominator_present_text_before,
    CursorSide::Numerator,
    NeighborContext::TextBefore,
    true,
    true
);
fraction_start_test!(
    numerator_present_denominator_present_text_after,
    CursorSide::Numerator,
    NeighborContext::TextAfter,
    true,
    true
);
fraction_start_test!(
    numerator_present_denominator_present_text_both_sides,
    CursorSide::Numerator,
    NeighborContext::TextOnBothSides,
    true,
    true
);
fraction_start_test!(
    numerator_present_denominator_present_operators_both_sides,
    CursorSide::Numerator,
    NeighborContext::OperatorsOnBothSides,
    true,
    true
);
fraction_start_test!(
    numerator_present_denominator_present_operator_before_text_after,
    CursorSide::Numerator,
    NeighborContext::OperatorBeforeTextAfter,
    true,
    true
);
fraction_start_test!(
    numerator_present_denominator_present_text_before_operator_after,
    CursorSide::Numerator,
    NeighborContext::TextBeforeOperatorAfter,
    true,
    true
);

fraction_start_test!(
    denominator_empty_empty_no_siblings,
    CursorSide::Denominator,
    NeighborContext::NoSiblings,
    false,
    false
);
fraction_start_test!(
    denominator_empty_empty_text_before,
    CursorSide::Denominator,
    NeighborContext::TextBefore,
    false,
    false
);
fraction_start_test!(
    denominator_empty_empty_text_after,
    CursorSide::Denominator,
    NeighborContext::TextAfter,
    false,
    false
);
fraction_start_test!(
    denominator_empty_empty_text_both_sides,
    CursorSide::Denominator,
    NeighborContext::TextOnBothSides,
    false,
    false
);
fraction_start_test!(
    denominator_empty_empty_operators_both_sides,
    CursorSide::Denominator,
    NeighborContext::OperatorsOnBothSides,
    false,
    false
);
fraction_start_test!(
    denominator_empty_empty_operator_before_text_after,
    CursorSide::Denominator,
    NeighborContext::OperatorBeforeTextAfter,
    false,
    false
);
fraction_start_test!(
    denominator_empty_empty_text_before_operator_after,
    CursorSide::Denominator,
    NeighborContext::TextBeforeOperatorAfter,
    false,
    false
);
fraction_start_test!(
    denominator_empty_denominator_no_siblings,
    CursorSide::Denominator,
    NeighborContext::NoSiblings,
    false,
    true
);
fraction_start_test!(
    denominator_empty_denominator_text_before,
    CursorSide::Denominator,
    NeighborContext::TextBefore,
    false,
    true
);
fraction_start_test!(
    denominator_empty_denominator_text_after,
    CursorSide::Denominator,
    NeighborContext::TextAfter,
    false,
    true
);
fraction_start_test!(
    denominator_empty_denominator_text_both_sides,
    CursorSide::Denominator,
    NeighborContext::TextOnBothSides,
    false,
    true
);
fraction_start_test!(
    denominator_empty_denominator_operators_both_sides,
    CursorSide::Denominator,
    NeighborContext::OperatorsOnBothSides,
    false,
    true
);
fraction_start_test!(
    denominator_empty_denominator_operator_before_text_after,
    CursorSide::Denominator,
    NeighborContext::OperatorBeforeTextAfter,
    false,
    true
);
fraction_start_test!(
    denominator_empty_denominator_text_before_operator_after,
    CursorSide::Denominator,
    NeighborContext::TextBeforeOperatorAfter,
    false,
    true
);
fraction_start_test!(
    denominator_present_empty_no_siblings,
    CursorSide::Denominator,
    NeighborContext::NoSiblings,
    true,
    false
);
fraction_start_test!(
    denominator_present_empty_text_before,
    CursorSide::Denominator,
    NeighborContext::TextBefore,
    true,
    false
);
fraction_start_test!(
    denominator_present_empty_text_after,
    CursorSide::Denominator,
    NeighborContext::TextAfter,
    true,
    false
);
fraction_start_test!(
    denominator_present_empty_text_both_sides,
    CursorSide::Denominator,
    NeighborContext::TextOnBothSides,
    true,
    false
);
fraction_start_test!(
    denominator_present_empty_operators_both_sides,
    CursorSide::Denominator,
    NeighborContext::OperatorsOnBothSides,
    true,
    false
);
fraction_start_test!(
    denominator_present_empty_operator_before_text_after,
    CursorSide::Denominator,
    NeighborContext::OperatorBeforeTextAfter,
    true,
    false
);
fraction_start_test!(
    denominator_present_empty_text_before_operator_after,
    CursorSide::Denominator,
    NeighborContext::TextBeforeOperatorAfter,
    true,
    false
);
fraction_start_test!(
    denominator_present_denominator_present_no_siblings,
    CursorSide::Denominator,
    NeighborContext::NoSiblings,
    true,
    true
);
fraction_start_test!(
    denominator_present_denominator_present_text_before,
    CursorSide::Denominator,
    NeighborContext::TextBefore,
    true,
    true
);
fraction_start_test!(
    denominator_present_denominator_present_text_after,
    CursorSide::Denominator,
    NeighborContext::TextAfter,
    true,
    true
);
fraction_start_test!(
    denominator_present_denominator_present_text_both_sides,
    CursorSide::Denominator,
    NeighborContext::TextOnBothSides,
    true,
    true
);
fraction_start_test!(
    denominator_present_denominator_present_operators_both_sides,
    CursorSide::Denominator,
    NeighborContext::OperatorsOnBothSides,
    true,
    true
);
fraction_start_test!(
    denominator_present_denominator_present_operator_before_text_after,
    CursorSide::Denominator,
    NeighborContext::OperatorBeforeTextAfter,
    true,
    true
);
fraction_start_test!(
    denominator_present_denominator_present_text_before_operator_after,
    CursorSide::Denominator,
    NeighborContext::TextBeforeOperatorAfter,
    true,
    true
);

fn assert_inside_backspace_is_forwarded(side: CursorSide) {
    let (mut tree, container_id, numerator, denominator) =
        make_fraction(true, true, vec![], vec![]);
    let sequence_id = match side {
        CursorSide::Numerator => numerator,
        CursorSide::Denominator => denominator,
    };
    let mut cursor = Cursor::new_raw(sequence_id, Some((0, Some(0))));

    let result = tree
        .get_mut_container(container_id)
        .unwrap()
        .handle_event(event::Event::Backspace, &mut cursor);

    assert!(matches!(result, Some(event::Event::Backspace)));
    assert!(tree.get_container(container_id).is_some());
    assert_eq!(tree.get_sequence(numerator).unwrap().child_count(), 1);
    assert_eq!(tree.get_sequence(denominator).unwrap().child_count(), 1);
    assert_eq!(cursor.sequence_id(), sequence_id);
    assert_eq!(cursor.child_index(), Some((0, Some(0))));
}

#[test]
fn backspace_inside_numerator_is_forwarded_without_flattening() {
    assert_inside_backspace_is_forwarded(CursorSide::Numerator);
}

#[test]
fn backspace_inside_denominator_is_forwarded_without_flattening() {
    assert_inside_backspace_is_forwarded(CursorSide::Denominator);
}

fn assert_operator_backspace_is_forwarded(side: CursorSide) {
    let (mut tree, container_id, numerator, denominator) =
        make_fraction(false, false, vec![], vec![]);
    let sequence_id = match side {
        CursorSide::Numerator => numerator,
        CursorSide::Denominator => denominator,
    };
    tree.get_mut_sequence(sequence_id)
        .unwrap()
        .add_atom(Atom::Operator(crate::tree::OperatorKind::Add));
    let mut cursor = Cursor::new_raw(sequence_id, Some((0, None)));
    let result = tree
        .get_mut_container(container_id)
        .unwrap()
        .handle_event(event::Event::Backspace, &mut cursor);

    assert!(matches!(result, Some(event::Event::Backspace)));
    assert!(tree.get_container(container_id).is_some());
    assert_eq!(tree.get_sequence(sequence_id).unwrap().child_count(), 1);
    assert_eq!(cursor.sequence_id(), sequence_id);
    assert_eq!(cursor.child_index(), Some((0, None)));
}

#[test]
fn backspace_on_numerator_operator_is_forwarded() {
    assert_operator_backspace_is_forwarded(CursorSide::Numerator);
}

#[test]
fn backspace_on_denominator_operator_is_forwarded() {
    assert_operator_backspace_is_forwarded(CursorSide::Denominator);
}

#[test]
fn backspace_with_a_cursor_outside_the_fraction_is_forwarded() {
    let (mut tree, container_id, numerator, denominator) =
        make_fraction(true, true, vec![], vec![]);
    let root_id = tree.root_id();
    let mut cursor = Cursor::new_raw(root_id, None);

    let result = tree
        .get_mut_container(container_id)
        .unwrap()
        .handle_event(event::Event::Backspace, &mut cursor);

    assert!(matches!(result, Some(event::Event::Backspace)));
    assert!(tree.get_container(container_id).is_some());
    assert!(tree.get_sequence(numerator).is_some());
    assert!(tree.get_sequence(denominator).is_some());
    assert_eq!(cursor.sequence_id(), root_id);
    assert_eq!(cursor.child_index(), None);
}

fn assert_backspace_flattens_fraction_inside_parent(parent_in_denominator: bool) {
    let mut tree = Tree::new();
    let outer_id = tree
        .get_mut_sequence(tree.root_id())
        .unwrap()
        .add_container(ContainerKind::fraction)
        .unwrap();
    let (outer_numerator, outer_denominator) = match tree.get_container(outer_id).unwrap().kind() {
        ContainerKind::Fraction {
            numerator,
            denominator,
        } => (*numerator, *denominator),
    };
    let parent_sequence_id = if parent_in_denominator {
        outer_denominator
    } else {
        outer_numerator
    };
    let (inner_id, numerator, denominator) = {
        let mut parent = tree.get_mut_sequence(parent_sequence_id).unwrap();
        parent.add_atom(Atom::Text("P".into()));
        let inner_id = parent.add_container(ContainerKind::fraction).unwrap();
        parent.add_atom(Atom::Text("S".into()));
        let (numerator, denominator) = match parent.tree().get_container(inner_id).unwrap().kind() {
            ContainerKind::Fraction {
                numerator,
                denominator,
            } => (*numerator, *denominator),
        };
        (inner_id, numerator, denominator)
    };
    tree.get_mut_sequence(numerator)
        .unwrap()
        .add_atom(Atom::Text("N".into()));
    tree.get_mut_sequence(denominator)
        .unwrap()
        .add_atom(Atom::Text("D".into()));
    let mut cursor = Cursor::new_raw(numerator, None);

    let result = tree
        .get_mut_container(inner_id)
        .unwrap()
        .handle_event(event::Event::Backspace, &mut cursor);

    assert!(result.is_none());
    assert!(tree.get_container(outer_id).is_some());
    assert!(tree.get_container(inner_id).is_none());
    assert_eq!(cursor.sequence_id(), parent_sequence_id);
    assert!(matches!(
        tree.get_sequence(parent_sequence_id)
            .unwrap()
            .get_child(0),
        Some(ChildRef::Atom(Atom::Text(text))) if text == "PNDS"
    ));
    assert_eq!(
        tree.get_sequence(parent_sequence_id).unwrap().child_count(),
        1
    );
}

#[test]
fn backspace_flattens_fraction_inside_parent_numerator() {
    assert_backspace_flattens_fraction_inside_parent(false);
}

#[test]
fn backspace_flattens_fraction_inside_parent_denominator() {
    assert_backspace_flattens_fraction_inside_parent(true);
}

#[test]
fn flattening_fraction_preserves_neighboring_fraction_containers() {
    let mut tree = Tree::new();
    let (before_id, target_id, after_id, numerator, denominator) = {
        let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
        let before_id = root.add_container(ContainerKind::fraction).unwrap();
        let target_id = root.add_container(ContainerKind::fraction).unwrap();
        let after_id = root.add_container(ContainerKind::fraction).unwrap();
        let (numerator, denominator) = match root.tree().get_container(target_id).unwrap().kind() {
            ContainerKind::Fraction {
                numerator,
                denominator,
            } => (*numerator, *denominator),
        };
        (before_id, target_id, after_id, numerator, denominator)
    };
    tree.get_mut_sequence(numerator)
        .unwrap()
        .add_atom(Atom::Text("N".into()));
    tree.get_mut_sequence(denominator)
        .unwrap()
        .add_atom(Atom::Text("D".into()));
    let mut cursor = Cursor::new_raw(numerator, None);

    let result = tree
        .get_mut_container(target_id)
        .unwrap()
        .handle_event(event::Event::Backspace, &mut cursor);

    assert!(result.is_none());
    assert!(tree.get_container(target_id).is_none());
    assert_eq!(tree.root().child_count(), 3);
    assert!(matches!(
        tree.root().get_child(0),
        Some(ChildRef::Container(container)) if container.id() == before_id
    ));
    assert!(matches!(
        tree.root().get_child(1),
        Some(ChildRef::Atom(Atom::Text(text))) if text == "ND"
    ));
    assert!(matches!(
        tree.root().get_child(2),
        Some(ChildRef::Container(container)) if container.id() == after_id
    ));
}

#[test]
fn flattening_fraction_preserves_mixed_children_in_order() {
    let (mut tree, container_id, numerator, denominator) = make_fraction(
        false,
        false,
        vec![Atom::Text("L".into())],
        vec![Atom::Text("R".into())],
    );
    tree.get_mut_sequence(numerator)
        .unwrap()
        .add_atom(Atom::Text("N".into()));
    tree.get_mut_sequence(numerator)
        .unwrap()
        .add_atom(Atom::Operator(crate::tree::OperatorKind::Add));
    tree.get_mut_sequence(denominator)
        .unwrap()
        .add_atom(Atom::Text("D".into()));
    tree.get_mut_sequence(denominator)
        .unwrap()
        .add_atom(Atom::Operator(crate::tree::OperatorKind::Sub));
    let mut cursor = Cursor::new_raw(numerator, None);

    let result = tree
        .get_mut_container(container_id)
        .unwrap()
        .handle_event(event::Event::Backspace, &mut cursor);

    assert!(result.is_none());
    let actual: Vec<_> = tree
        .root()
        .children()
        .into_iter()
        .map(|child| match child {
            ChildRef::Atom(atom) => atom.clone(),
            ChildRef::Container(_) => panic!("fixture contains no nested containers"),
        })
        .collect();
    assert_eq!(
        actual,
        vec![
            Atom::Text("LN".into()),
            Atom::Operator(crate::tree::OperatorKind::Add),
            Atom::Text("D".into()),
            Atom::Operator(crate::tree::OperatorKind::Sub),
            Atom::Text("R".into()),
        ]
    );
}

#[test]
fn flattening_fraction_restores_numerator_cursor_to_previous_grapheme() {
    let (mut tree, container_id, numerator, _) =
        make_fraction(true, false, vec![Atom::Text("é".into())], vec![]);
    let mut cursor = Cursor::new_raw(numerator, None);

    let result = tree
        .get_mut_container(container_id)
        .unwrap()
        .handle_event(event::Event::Backspace, &mut cursor);

    assert!(result.is_none());
    assert!(matches!(
        tree.root().get_child(0),
        Some(ChildRef::Atom(Atom::Text(text))) if text == "éN"
    ));
    assert_eq!(cursor.child_index(), Some((0, Some(0))));
}

#[test]
fn flattening_fraction_restores_denominator_cursor_to_previous_grapheme() {
    let (mut tree, container_id, numerator, denominator) =
        make_fraction(false, false, vec![], vec![]);
    tree.get_mut_sequence(numerator)
        .unwrap()
        .add_atom(Atom::Text("abc".into()));
    tree.get_mut_sequence(denominator)
        .unwrap()
        .add_atom(Atom::Text("def".into()));
    let mut cursor = Cursor::new_raw(denominator, None);

    let result = tree
        .get_mut_container(container_id)
        .unwrap()
        .handle_event(event::Event::Backspace, &mut cursor);

    assert!(result.is_none());
    assert!(matches!(
        tree.root().get_child(0),
        Some(ChildRef::Atom(Atom::Text(text))) if text == "abcdef"
    ));
    assert_eq!(cursor.child_index(), Some((0, Some(2))));
}

#[test]
fn flattening_fraction_restores_denominator_cursor_after_parent_text_join() {
    let (mut tree, container_id, _, denominator) =
        make_fraction(true, true, vec![Atom::Text("P".into())], vec![]);
    let mut cursor = Cursor::new_raw(denominator, None);

    let result = tree
        .get_mut_container(container_id)
        .unwrap()
        .handle_event(event::Event::Backspace, &mut cursor);

    assert!(result.is_none());
    assert!(matches!(
        tree.root().get_child(0),
        Some(ChildRef::Atom(Atom::Text(text))) if text == "PND"
    ));
    assert_eq!(cursor.child_index(), Some((0, Some(1))));
}

#[test]
fn flattening_fraction_restores_denominator_cursor_to_previous_child() {
    let (mut tree, container_id, numerator, denominator) =
        make_fraction(false, false, vec![Atom::Operator(crate::tree::OperatorKind::Add)], vec![]);
    tree.get_mut_sequence(numerator)
        .unwrap()
        .add_atom(Atom::Operator(crate::tree::OperatorKind::Mul));
    tree.get_mut_sequence(denominator)
        .unwrap()
        .add_atom(Atom::Operator(crate::tree::OperatorKind::Sub));
    tree.get_mut_sequence(denominator)
        .unwrap()
        .add_atom(Atom::Text("text".into()));
    let mut cursor = Cursor::new_raw(denominator, None);

    let result = tree
        .get_mut_container(container_id)
        .unwrap()
        .handle_event(event::Event::Backspace, &mut cursor);

    assert!(result.is_none());
    assert_eq!(cursor.child_index(), Some((1, None)));
    assert!(matches!(
        tree.root().get_child(1),
        Some(ChildRef::Atom(Atom::Operator(crate::tree::OperatorKind::Mul)))
    ));
}

#[test]
fn flattening_fraction_with_nested_fraction_children_keeps_them_in_parent() {
    let (mut tree, container_id, numerator, denominator) =
        make_fraction(false, true, vec![], vec![]);
    let nested_id = tree
        .get_mut_sequence(numerator)
        .unwrap()
        .add_container(ContainerKind::fraction)
        .unwrap();
    let mut cursor = Cursor::new_raw(numerator, None);

    let result = tree
        .get_mut_container(container_id)
        .unwrap()
        .handle_event(event::Event::Backspace, &mut cursor);

    assert!(result.is_none());
    assert!(tree.get_container(container_id).is_none());
    assert!(tree.get_sequence(numerator).is_none());
    assert!(tree.get_sequence(denominator).is_none());
    assert_eq!(tree.root().child_count(), 2);
    assert!(matches!(
        tree.root().get_child(0),
        Some(ChildRef::Container(nested)) if nested.id() == nested_id
    ));
    assert_eq!(
        tree.get_container(nested_id).unwrap().parent_id(),
        tree.root_id()
    );
}

fn coalesce_text(atoms: Vec<Atom>) -> Vec<Atom> {
    let mut result: Vec<Atom> = Vec::new();
    for atom in atoms {
        if let (Some(Atom::Text(previous)), Atom::Text(text)) = (result.last_mut(), &atom) {
            previous.push_str(text);
        } else {
            result.push(atom);
        }
    }
    result
}

fn assert_cursor_is_valid(tree: &Tree, cursor: Cursor) {
    if let Some((child_index, text_index)) = cursor.child_index() {
        let child = tree
            .root()
            .get_child(child_index)
            .expect("cursor child index must exist");
        match child {
            ChildRef::Atom(Atom::Text(text)) => {
                if let Some(text_index) = text_index {
                    assert!(
                        text_index < text.chars().count(),
                        "cursor text index must be within its text child"
                    );
                }
            }
            ChildRef::Atom(Atom::Operator(_)) | ChildRef::Container(_) => {
                assert!(text_index.is_none());
            }
        }
    }
}
