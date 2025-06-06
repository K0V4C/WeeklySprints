use std::ops::{Deref, Range};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub type GraphemeIdx = usize;
pub type LineIdx = usize;
pub type ByteIdx = usize;

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
#[derive(Clone)]
pub struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>,
    start_byte_idx: ByteIdx,
}

#[derive(Default, Clone)]
pub struct Line {
    fragments: Vec<TextFragment>,
    string: String,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        let fragments = Self::str_to_fragments(line_str);
        Line {
            fragments,
            string: String::from(line_str),
        }
    }

    pub fn concat(&mut self, concat_line: &Line) {
        self.string.push_str(&concat_line.string);
        self.rebuild_fragments();
    }

    pub fn split_off(&mut self, at: GraphemeIdx) -> Line {
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

    // ========================================================= Find ==================================================================
    fn grapheme_idx_to_byte_idx(&self, grapheme_idx: GraphemeIdx) -> ByteIdx {
        self.fragments
            .get(grapheme_idx)
            .map_or(0, |fragment| fragment.start_byte_idx)
    }

    fn byte_idx_to_grapheme_idx(&self, byte_idx: ByteIdx) -> GraphemeIdx {
        self.fragments
            .iter()
            .position(|fragment| fragment.start_byte_idx >= byte_idx)
            .map_or(0, |grapheme_idx| grapheme_idx)
    }

    pub fn backward_find(
        &self,
        search_string: &str,
        end_point: GraphemeIdx,
    ) -> Option<GraphemeIdx> {
        if end_point >= self.string.len() {
            return None;
        }

        // TODO: need to fix this
        let slice = &self.string[..=self.grapheme_idx_to_byte_idx(end_point)];

        slice
            .rmatch_indices(search_string)
            .next()
            .map(|(byte_idx, _)| self.byte_idx_to_grapheme_idx(byte_idx))
    }

    pub fn forward_find(
        &self,
        search_string: &str,
        start_look_point: GraphemeIdx,
    ) -> Option<GraphemeIdx> {
        if start_look_point >= self.string.len() {
            return None;
        }

        let slice = &self.string[self.grapheme_idx_to_byte_idx(start_look_point)..];

        slice
            .match_indices(search_string)
            .next()
            .map(|(byte_idx, _)| {
                self.byte_idx_to_grapheme_idx(byte_idx.saturating_add(start_look_point))
            })
    }

    pub fn get_next_match_idx(&self, start_idx: GraphemeIdx, search_string: &str) -> GraphemeIdx {
        if start_idx >= self.grapheme_count() {
            return 1;
        }

        let grapheme_len = Line::from(search_string).grapheme_count();

        if (start_idx + grapheme_len) >= self.grapheme_count() {
            return self.grapheme_count();
        }

        if start_idx + grapheme_len >= self.string.len() {}

        let slice =
            &self.string[self.grapheme_idx_to_byte_idx(start_idx.saturating_add(grapheme_len))..];

        slice.match_indices(search_string).next().map_or(1, |x| {
            self.byte_idx_to_grapheme_idx(x.0.saturating_add(grapheme_len))
        })
    }
    // ========================================================== String manipulation ==================================================

    pub fn add_character_to_line(&mut self, chr: char, at: GraphemeIdx) {
        if let Some(fragment) = self.fragments.get(at) {
            self.string.insert(fragment.start_byte_idx, chr);
        } else {
            self.string.push(chr);
        }
        self.rebuild_fragments();
    }

    pub fn delete_character(&mut self, at: GraphemeIdx) {
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
    pub fn find_all(&self, query: &str, range: Range<ByteIdx>) -> Vec<(ByteIdx, GraphemeIdx)> {
        let end = std::cmp::min(range.end, self.string.len());
        let start = range.start;
        debug_assert!(start <= end);
        debug_assert!(start <= self.string.len());
        self.string.get(start..end).map_or_else(Vec::new, |substr| {
            let potential_matches: Vec<ByteIdx> = substr
                .match_indices(query) // find _potential_ matches within the substring
                .map(|(relative_start_idx, _)| {
                    relative_start_idx.saturating_add(start) //convert their relative indices to absolute indices
                })
                .collect();
            self.match_graphme_clusters(&potential_matches, query) //convert the potential matches into matches which align with the grapheme boundaries.
        })
    }

    pub fn get_visable_graphemes(&self, range: Range<GraphemeIdx>) -> String {
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

    pub fn grapheme_count(&self) -> GraphemeIdx {
        self.fragments.len()
    }

    pub fn width_until(&self, grapheme_index: GraphemeIdx) -> GraphemeIdx {
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

    // =====================================================  HELPER METHODS =========================================================

    // Finds all matches which align with grapheme boundaries.
    // Parameters:
    // - query: The query to search for.
    // - matches: A vector of byte indices of potential matches, which might or might not align with the grapheme clusters.
    // Returns:
    // A Vec of (byte_index, grapheme_idx) pairs for each match that alignes with the grapheme clusters, where byte_index is the byte index of the match, and grapheme_idx is the grapheme index of the match.
    fn match_graphme_clusters(
        &self,
        matches: &[ByteIdx],
        query: &str,
    ) -> Vec<(ByteIdx, GraphemeIdx)> {
        let grapheme_count = query.graphemes(true).count();
        matches
            .iter()
            .filter_map(|&start| {
                let grapheme_idx = self.byte_idx_to_grapheme_idx(start);
                self.fragments
                    .get(grapheme_idx..grapheme_idx.saturating_add(grapheme_count)) // get all fragments that should be part of the match
                    .and_then(|fragments| {
                        let substring = fragments
                            .iter()
                            .map(|fragment| fragment.grapheme.as_str())
                            .collect::<String>(); //combine the fragments into a single string.
                        (substring == query).then_some((start, grapheme_idx))
                        // if the combined string matches the query, we have an actual match.
                    })
            })
            .collect()
    }

    pub fn clear(&mut self) {
        self.fragments.clear();
        self.string.clear();
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

impl Deref for Line {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.string
    }
}
