use super::*;

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
