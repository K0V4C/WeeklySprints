use std::ops::Range;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Copy, Clone, Debug)]
pub enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    fn saturating_add(self, other: usize) -> usize {
        match self {
            GraphemeWidth::Half => other.saturating_add(1),
            GraphemeWidth::Full => other.saturating_add(2),
        }
    }
}

pub struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>,
    start_byte_idx: usize,
}

#[derive(Default)]
pub struct Line {
    fragments: Vec<TextFragment>,
    string: String
}

impl Line {

    pub fn from(line_str: &str) -> Self {
        let fragments = Self::str_to_fragments(line_str);
        Line { fragments, string: String::from(line_str)}
    }

    pub fn concat(&mut self, concat_line: &Line) {
        self.string.push_str(&concat_line.string);
        self.rebuild_fragments();
    }

    pub fn split_off(&mut self, at: usize) -> Line {
        if let Some(fragment) = self.fragments.get(at) {
            let remainder = self.string.split_off(fragment.start_byte_idx);
            self.rebuild_fragments();
            Self::from(&remainder)
        } else {
            Self::default()
        }
    }

    // ========================================================= Builders =============================================================

    fn rebuild_fragments(&mut self) {
        self.fragments = Self::str_to_fragments(&self.to_string());
    }

    // ========================================================== String manipulation ==================================================

    pub fn add_character_to_line(&mut self, chr: char, at: usize) {
        if let Some(fragment) = self.fragments.get(at) {
            self.string.insert(fragment.start_byte_idx, chr);
        } else {
            self.string.push(chr);
        }
        self.rebuild_fragments();
    }

    pub fn delete_character(&mut self, at: usize) {
        if let Some(fragment) = self.fragments.get(at) {
            let start = fragment.start_byte_idx;
            let end = fragment
                .start_byte_idx
                .saturating_add(fragment.grapheme.len());
            self.string.drain(start..end);
            self.rebuild_fragments();
        }
    }

    // ============================================================= Getters =====================================================

    pub fn get_visable_graphemes(&self, range: Range<usize>) -> String {
        if range.start >= range.end {
            return String::new();
        }

        let mut result = String::new();
        let mut current_pos = 0;

        for fragment in &self.fragments {
            let fragment_end = fragment.rendered_width.saturating_add(current_pos);

            if current_pos >= range.end {
                break;
            }

            if fragment_end > range.start {
                if fragment_end > range.end || current_pos < range.start {
                    result.push('⋯');
                } else if let Some(char) = fragment.replacement {
                    result.push(char);
                } else {
                    result.push_str(&fragment.grapheme);
                }
            }

            current_pos = fragment_end;
        }

        result
    }

    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

    pub fn width_until(&self, grapheme_index: usize) -> usize {
        self.fragments
            .iter()
            .take(grapheme_index)
            .map(|fragment| match fragment.rendered_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            })
            .sum()
    }
    fn get_replacement_character(grapheme: &str) -> Option<char> {
        let width = grapheme.width();

        match grapheme {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && grapheme.trim().is_empty() => Some('␣'),
            _ if width == 0 => {
                let mut chars = grapheme.chars();
                if let Some(ch) = chars.next() {
                    if ch.is_control() && chars.next().is_none() {
                        return Some('▯');
                    }
                }
                Some('·')
            }

            _ => None,
        }
    }

    ///////////////////////////////////////////// HELPER METHODS ////////////////////////////////////////////////

    pub fn clear(&mut self) {
        self.fragments.clear();
    }

    fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
        line_str
                .grapheme_indices(true)
                .map(|(byte_idx, grapheme)| {
                    let (replacement, rendered_width) = Self::get_replacement_character(grapheme)
                        .map_or_else(
                            || {
                                let unicode_width = grapheme.width();
                                let rendered_width = match unicode_width {
                                    0 | 1 => GraphemeWidth::Half,
                                    _ => GraphemeWidth::Full,
                                };
                                (None, rendered_width)
                            },
                            |replacement| (Some(replacement), GraphemeWidth::Half),
                        );

                    TextFragment {
                        grapheme: grapheme.to_string(),
                        rendered_width,
                        replacement,
                        start_byte_idx: byte_idx,
                    }
                })
                .collect()
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}
