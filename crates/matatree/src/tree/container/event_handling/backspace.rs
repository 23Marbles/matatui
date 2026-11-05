mod cursor;

#[cfg(test)]
mod tests;

use crate::editor::{cursor::Cursor, event};

use super::super::*;

pub(super) fn handle(container: ContainerMut<'_>, cursor: &mut Cursor) -> Option<event::Event> {
    let ContainerKind::Fraction {
        numerator,
        denominator,
    } = container.kind;

    let parent_id = container.parent_id;

    if cursor.child_index() != None
        || (cursor.sequence_id() != numerator && cursor.sequence_id() != denominator)
    {
        return Some(event::Event::Backspace);
    }

    let mut numerator_children = container.tree.sequences.remove(numerator).unwrap().children;
    let num_len = numerator_children.len();

    for child in numerator_children.iter_mut() {
        match child {
            Child::Atom(_) => (),
            Child::Container(container_id) => container.tree.get_mut_container(*container_id).unwrap().parent_id = parent_id,
        }
    }

    let mut denominator_children = container
        .tree
        .sequences
        .remove(denominator)
        .unwrap()
        .children;
    let denom_len = denominator_children.len();

    for child in denominator_children.iter_mut() {
        match child {
            Child::Atom(_) => (),
            Child::Container(container_id) => container.tree.get_mut_container(*container_id).unwrap().parent_id = parent_id,
        }
    }

    let id = container.id;

    let mut parent = container.into_parent_mut();
    let children = &mut parent.children;

    let parent_idx = children
        .iter()
        .enumerate()
        .find(|(_, child)| match child {
            Child::Atom(_) => false,
            Child::Container(container_id) => *container_id == id,
        })
        .unwrap()
        .0;

    let _ = children.splice(
        parent_idx..=parent_idx,
        numerator_children
            .into_iter()
            .chain(denominator_children.into_iter()),
    );

    if denom_len != 0 {
        let r = join_text_children(children, parent_idx + num_len + denom_len);
        log::debug!("at_end {r:?}")
    };

    let before_denom_text_pos = if num_len != 0 {
        join_text_children(children, parent_idx + num_len)
    } else {
        None
    };

    let before_num_text_pos = join_text_children(children, parent_idx);

    log::debug!("before_denom {before_denom_text_pos:?}");
    log::debug!("before_num {before_num_text_pos:?}");

    cursor::restore(
        cursor,
        parent_id,
        parent_idx,
        num_len,
        before_num_text_pos,
        before_denom_text_pos,
        numerator,
        denominator,
        children,
    );

    parent.into_tree_mut().containers.remove(id);
    None
}
