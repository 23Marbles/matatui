use super::*;

#[must_use]
pub(super) fn join_text_children(children: &mut Vec<Child>, join_seam: usize) -> Option<usize> {
    if let Some(prev) = join_seam
        .checked_sub(1)
        .and_then(|index| children.get(index))
        && let Some(after) = children.get(join_seam)
    {
        if matches!(prev, Child::Atom(Atom::Text(_))) && matches!(after, Child::Atom(Atom::Text(_)))
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
    }

    None
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Atom {
    Text(String),

    Operator(char),
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
