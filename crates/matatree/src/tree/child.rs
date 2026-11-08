use std::fmt::{Display, Write};

use super::*;

#[must_use]
pub(super) fn join_text_children(children: &mut Vec<Child>, join_seam: usize) -> Option<usize> {
    if let Some(prev) = join_seam
        .checked_sub(1)
        .and_then(|index| children.get(index))
        && let Some(after) = children.get(join_seam)
        && matches!(prev, Child::Atom(Atom::Text(_))) && matches!(after, Child::Atom(Atom::Text(_)))
        {
            let Child::Atom(Atom::Text(after_text)) = children.remove(join_seam) else {
                unreachable!("Already checked")
            };

            let Some(Child::Atom(Atom::Text(prev_text))) = children.get_mut(join_seam - 1) else {
                unreachable!("Already checked")
            };

            let res = Some(prev_text.len());

            prev_text.push_str(&after_text);

            return res;
        }

    None
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Atom {
    Text(String),

    Operator(OperatorKind),
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OperatorKind {
    Mul,
    Add,
    Sub,
    GreaterThan,
    LessThan,
    Equal,
}

impl OperatorKind {
    pub fn from_char(ch: char) -> Option<Self> {
        Some(match ch {
            '*' => Self::Mul,
            '+' => Self::Add,
            '-' => Self::Sub,
            '>' => Self::GreaterThan,
            '<' => Self::LessThan,
            '=' => Self::Equal,
            _ => return None,
        })
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Mul => '×',
            Self::Add => '+',
            Self::Sub => '-',
            Self::GreaterThan => '>',
            Self::LessThan => '<',
            Self::Equal => '=',
        }
    }
}

impl Display for OperatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.to_char())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Child {
    Atom(Atom),

    Container(ContainerId),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChildRef<'a> {
    Atom(&'a Atom),

    Container(ContainerRef<'a>),
}

pub enum ChildMut<'a> {
    Atom(&'a mut Atom),

    Container(&'a mut Container),
}

#[derive(Debug, Clone)]
pub enum OwnedChild {
    Atom(Atom),

    Container(OwnedContainer),
}

pub struct Children<'a> {
    parent: SequenceRef<'a>,
}

impl<'a> Children<'a> {
    pub fn new(parent: SequenceRef<'a>) -> Self {
        Self { parent }
    }
}

impl<'a> IntoIterator for Children<'a> {
    type Item = ChildRef<'a>;

    type IntoIter = ChildIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ChildIter::new(self.parent)
    }
}

pub struct ChildIter<'a> {
    parent: SequenceRef<'a>,

    next_index: usize,
}

impl<'a> ChildIter<'a> {
    pub fn new(parent: SequenceRef<'a>) -> Self {
        Self {
            parent,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for ChildIter<'a> {
    type Item = ChildRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let child = self.parent.get_child(self.next_index)?;
        self.next_index += 1;
        Some(child)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn child_iterator_yields_atoms_and_nested_containers_in_order() {
        let mut tree = Tree::new();
        let (container_id, numerator) = {
            let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
            root.add_atom(Atom::Text("before".into()));
            let id = root
                .add_container(|[numerator, denominator]| {
                    ContainerKind::fraction([numerator, denominator])
                })
                .unwrap();
            root.add_atom(Atom::Operator(OperatorKind::Add));
            let ContainerKind::Fraction { numerator, .. } =
                root.tree().get_container(id).unwrap().kind().clone();
            (id, numerator)
        };
        tree.get_mut_sequence(numerator)
            .unwrap()
            .add_atom(Atom::Text("n".into()));

        let children: Vec<_> = tree.root().children().into_iter().collect();

        assert!(matches!(&children[0], ChildRef::Atom(Atom::Text(text)) if text == "before"));
        assert!(
            matches!(&children[1], ChildRef::Container(container) if container.id() == container_id)
        );
        assert!(matches!(
            &children[2],
            ChildRef::Atom(Atom::Operator(OperatorKind::Add))
        ));
        assert_eq!(children.len(), 3);
        assert!(tree.root().get_child(3).is_none());
    }

    #[test]
    fn joining_text_children_only_joins_adjacent_text() {
        let mut children = vec![
            Child::Atom(Atom::Text("left".into())),
            Child::Atom(Atom::Text("right".into())),
        ];

        assert_eq!(join_text_children(&mut children, 1), Some(4));
        assert!(
            matches!(children.as_slice(), [Child::Atom(Atom::Text(text))] if text == "leftright")
        );
        assert_eq!(join_text_children(&mut children, 0), None);

        let mut separated = vec![
            Child::Atom(Atom::Text("left".into())),
            Child::Atom(Atom::Operator(OperatorKind::Add)),
            Child::Atom(Atom::Text("right".into())),
        ];
        assert_eq!(join_text_children(&mut separated, 1), None);
        assert_eq!(separated.len(), 3);
    }
}
