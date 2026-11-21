use matatree::{
    layout::LayoutEngine,
    render::Renderer,
    tree::{Atom, ContainerKind, Tree},
};

use super::*;

#[test]
fn renders_fraction_and_surrounding_atoms_on_expected_rows() {
    let mut tree = Tree::new();
    let fraction_id = {
        let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
        root.add_atom(Atom::Text("2".into()));
        root.add_atom(Atom::Operator(matatree::tree::OperatorKind::Add));
        let fraction_id = root.add_container(ContainerKind::fraction).unwrap();
        root.add_atom(Atom::Operator(matatree::tree::OperatorKind::Sub));
        root.add_atom(Atom::Text("5".into()));
        fraction_id
    };

    let (numerator, denominator) = match tree.get_container(fraction_id).unwrap().kind() {
        matatree::tree::ContainerKind::Fraction {
            numerator,
            denominator,
        } => (*numerator, *denominator),
        _ => {
            panic!("Expected fraction")
        }
    };
    tree.get_mut_sequence(numerator)
        .unwrap()
        .add_atom(Atom::Text("x".into()));
    tree.get_mut_sequence(denominator)
        .unwrap()
        .add_atom(Atom::Text("3".into()));

    let layout = RatatuiLayout::layout_sequence(tree.root(), None);
    let mut buf = ratatui::buffer::Buffer::empty(Rect::new(0, 0, 20, 10));

    RatatuiRender::render_layout(
        layout,
        Rect::new(0, 1, 20, 8),
        &mut buf,
        &RenderContext::none(),
    );

    for (x, y, expected) in [
        (0, 2, "2"),
        (2, 2, "+"),
        (5, 1, "x"),
        (4, 2, "─"),
        (5, 2, "─"),
        (6, 2, "─"),
        (5, 3, "3"),
        (8, 2, "-"),
        (10, 2, "5"),
    ] {
        assert_eq!(buf.cell((x, y)).unwrap().symbol(), expected);
    }
}

#[test]
fn renders_subscript_below_and_superscript_above_the_baseline() {
    let mut subscript_tree = Tree::new();
    let subscript_id = subscript_tree
        .get_mut_sequence(subscript_tree.root_id())
        .unwrap()
        .add_container(|[sequence]| ContainerKind::sub_script([sequence]))
        .unwrap();
    let subscript_sequence = match subscript_tree.get_container(subscript_id).unwrap().kind() {
        ContainerKind::SubScript { sequence } => *sequence,
        _ => panic!("Expected subscript"),
    };
    subscript_tree
        .get_mut_sequence(subscript_sequence)
        .unwrap()
        .add_atom(Atom::Text("2".into()));
    let subscript_layout = RatatuiLayout::layout_sequence(subscript_tree.root(), None);
    let mut subscript_buf = ratatui::buffer::Buffer::empty(Rect::new(0, 0, 5, 3));
    RatatuiRender::render_layout(
        subscript_layout,
        Rect::new(0, 0, 5, 3),
        &mut subscript_buf,
        &RenderContext::none(),
    );
    assert_eq!(subscript_buf.cell((0, 1)).unwrap().symbol(), "2");

    let mut superscript_tree = Tree::new();
    let superscript_id = superscript_tree
        .get_mut_sequence(superscript_tree.root_id())
        .unwrap()
        .add_container(|[sequence]| ContainerKind::super_script([sequence]))
        .unwrap();
    let superscript_sequence = match superscript_tree.get_container(superscript_id).unwrap().kind() {
        ContainerKind::SuperScript { sequence } => *sequence,
        _ => panic!("Expected superscript"),
    };
    superscript_tree
        .get_mut_sequence(superscript_sequence)
        .unwrap()
        .add_atom(Atom::Text("3".into()));
    let superscript_layout = RatatuiLayout::layout_sequence(superscript_tree.root(), None);
    let mut superscript_buf = ratatui::buffer::Buffer::empty(Rect::new(0, 0, 5, 3));
    RatatuiRender::render_layout(
        superscript_layout,
        Rect::new(0, 0, 5, 3),
        &mut superscript_buf,
        &RenderContext::none(),
    );
    assert_eq!(superscript_buf.cell((0, 0)).unwrap().symbol(), "3");
}
