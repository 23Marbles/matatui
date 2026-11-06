use crate::tree::{Atom, ChildMut, ChildRef, ContainerKind, OwnedChild, Tree};

#[test]
fn insertion_sets_fraction_sequence_parents_and_checks_index() {
    let mut tree = Tree::new();
    let (container_id, numerator, denominator) = {
        let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
        root.add_atom(Atom::Operator(crate::tree::OperatorKind::Add));
        let container_id = root
            .insert_container(1, |[numerator, denominator]| {
                ContainerKind::fraction([numerator, denominator])
            })
            .unwrap();
        let ContainerKind::Fraction {
            numerator,
            denominator,
        } = root
            .tree()
            .get_container(container_id)
            .unwrap()
            .kind()
            .clone();
        assert!(
            root.insert_container(3, |[numerator, denominator]| {
                ContainerKind::fraction([numerator, denominator])
            })
            .is_none()
        );
        (container_id, numerator, denominator)
    };

    assert_eq!(
        tree.get_sequence(numerator).unwrap().parent_id(),
        Some(container_id)
    );
    assert_eq!(
        tree.get_sequence(denominator).unwrap().parent_id(),
        Some(container_id)
    );
    assert_eq!(
        tree.get_container(container_id).unwrap().parent_id(),
        tree.root_id()
    );
    assert!(matches!(
        tree.root().get_child(0),
        Some(ChildRef::Atom(Atom::Operator(
            crate::tree::OperatorKind::Add
        )))
    ));
    assert!(matches!(
        tree.root().get_child(1),
        Some(ChildRef::Container(_))
    ));
}

#[test]
fn child_access_mutation_and_deletion_preserve_owned_atom() {
    let mut tree = Tree::new();
    {
        let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
        root.add_atom(Atom::Text("old".into()));
        assert!(matches!(root.get_child_mut(0), Some(ChildMut::Atom(_))));
        if let Some(ChildMut::Atom(Atom::Text(text))) = root.get_child_mut(0) {
            *text = "new".into();
        }
        assert!(matches!(
            root.into_child_mut(0),
            Some(ChildMut::Atom(Atom::Text(text))) if text == "new"
        ));
    }

    let removed = tree
        .get_mut_sequence(tree.root_id())
        .unwrap()
        .delete_child(0)
        .unwrap();

    assert!(matches!(
        removed,
        OwnedChild::Atom(Atom::Text(text)) if text == "new"
    ));
    assert!(tree.root().get_child(0).is_none());
    assert!(
        tree.get_mut_sequence(tree.root_id())
            .unwrap()
            .delete_child(0)
            .is_none()
    );
}

#[test]
fn insert_atom_preserves_child_order() {
    let mut tree = Tree::new();
    let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
    root.add_atom(Atom::Text("first".into()));
    root.add_atom(Atom::Text("last".into()));
    root.insert_atom(1, Atom::Operator(crate::tree::OperatorKind::Add));

    assert!(matches!(
        root.get_child(0),
        Some(ChildRef::Atom(Atom::Text(text))) if text == "first"
    ));
    assert!(matches!(
        root.get_child(1),
        Some(ChildRef::Atom(Atom::Operator(
            crate::tree::OperatorKind::Add
        )))
    ));
    assert!(matches!(
        root.get_child(2),
        Some(ChildRef::Atom(Atom::Text(text))) if text == "last"
    ));
}

#[test]
fn deleting_container_child_removes_its_subsequences() {
    let mut tree = Tree::new();
    let (container_id, numerator, denominator) = {
        let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
        let id = root
            .add_container(|[n, d]| ContainerKind::fraction([n, d]))
            .unwrap();
        let ContainerKind::Fraction {
            numerator,
            denominator,
        } = root.tree().get_container(id).unwrap().kind().clone();
        (id, numerator, denominator)
    };

    assert!(matches!(
        tree.get_mut_sequence(tree.root_id())
            .unwrap()
            .delete_child(0),
        Some(OwnedChild::Container(_))
    ));
    assert!(tree.get_container(container_id).is_none());
    assert!(tree.get_sequence(numerator).is_none());
    assert!(tree.get_sequence(denominator).is_none());
}
