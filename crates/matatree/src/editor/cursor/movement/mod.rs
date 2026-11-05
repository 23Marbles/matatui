mod container;
mod sequence;

use unicode_segmentation::UnicodeSegmentation;

pub(super) fn text_grapheme_count(text: &str) -> usize {
    text.graphemes(true).count()
}
