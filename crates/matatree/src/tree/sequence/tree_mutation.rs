use std::ops::Range;

use super::*;

impl Tree {
    pub fn drain_children(
        &mut self,
        src: SequenceId,
        range: impl Into<Range<usize>>,
    ) -> Option<Vec<Child>> {
        let src = self.sequences.get_mut(src)?;
        let range = range.into();

        if range.start > src.children.len()
            || range.end > src.children.len()
            || range.start > range.end
        {
            return None;
        }

        Some(src.children.drain(range).collect())
    }

    pub fn move_children(
        &mut self,
        src: SequenceId,
        src_range: impl Into<Range<usize>>,
        dst: SequenceId,
        dest_index: usize,
    ) {
        assert!(
            self.sequences
                .get(src)
                .map_or(false, |seq| dest_index <= seq.children.len())
        );

        let mut moved_children = self.drain_children(src, src_range).unwrap();

        for child in moved_children.iter_mut() {
            if let Child::Container(container_id) = child {
                let container = self.containers.get_mut(*container_id).unwrap();
                container.parent_id = dst;
            }
        }

        self.sequences
            .get_mut(dst)
            .unwrap()
            .children
            .splice(dest_index..dest_index, moved_children);
    }
}
