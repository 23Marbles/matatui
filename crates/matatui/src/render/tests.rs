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
        (0, 3, "2"),
        (2, 3, "+"),
        (5, 1, "x"),
        (4, 3, "─"),
        (5, 3, "─"),
        (6, 3, "─"),
        (5, 5, "3"),
        (8, 3, "-"),
        (10, 3, "5"),
    ] {
        assert_eq!(buf.cell((x, y)).unwrap().symbol(), expected);
    }
}
