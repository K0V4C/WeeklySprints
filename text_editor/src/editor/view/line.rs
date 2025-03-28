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
}

#[derive(Default)]
pub struct Line {
    fragments: Vec<TextFragment>,
}

impl Line {
    pub fn add_character_to_line(&mut self, chr: char, at: usize) {
        let mut result = String::new();

        for (index, fragment) in self.fragments.iter().enumerate() {
            if index == at {
                result.push(chr);
            }

            result.push_str(&fragment.grapheme);
        }

        if at >= self.fragments.len() {
            result.push(chr);
        }

        self.fragments = Self::str_to_fragments(&result);
    }

    pub fn delete_character(&mut self, at: usize) {
        let mut result = String::new();

        for (index, fragment) in self.fragments.iter().enumerate() {
            if index == at {
                continue;
            }

            result.push_str(&fragment.grapheme);
        }

        self.fragments = Self::str_to_fragments(&result);
    }

    pub fn concat(&mut self, concat_line: Line) {
        for fragment in concat_line.fragments {
            self.fragments.push(fragment);
        }
    }

    pub fn split_off(&mut self, at: usize) -> Line {

        if at > self.fragments.len() {
            return Line::default();
        }

        //W When calling split_off leftover part is [at, A)
        let cut_off = self.fragments.split_off(at);

        Line { fragments: cut_off }
    }

    pub fn from(line_str: &str) -> Self {
        let fragments = Self::str_to_fragments(line_str);
        Line { fragments }
    }

    fn replacement_character(grapheme: &str) -> Option<char> {
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

    ///////////////////////////////////////////// HELPER METHODS ////////////////////////////////////////////////

    fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
        let fragments = line_str
            .graphemes(true)
            .map(|grapheme| {
                let (rendered_width, replacement) = Self::replacement_character(grapheme)
                    .map_or_else(
                        || {
                            let width = grapheme.width();

                            let rendered_width = match width {
                                0 | 1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };

                            (rendered_width, None)
                        },
                        |replacement| (GraphemeWidth::Half, Some(replacement)),
                    );

                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                }
            })
            .collect();

        fragments
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let result: String = self.fragments.iter().map(|fragment| {
            fragment.grapheme.clone()
        }).collect();

        write!(f, "{result}")
    }
}
