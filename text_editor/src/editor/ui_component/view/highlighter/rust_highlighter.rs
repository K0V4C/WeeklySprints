use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

use crate::editor::{
    annotated_string::{annotation::Annotation, annotation_type::AnnotationType},
    line::{Line, LineIdx},
};

use super::syntax_highlihter::SyntaxHighlighter;

const KEYWORDS: [&str; 52] = [
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "async",
    "await",
    "dyn",
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",
    "try",
    "macro_rules",
    "union",
];

const TYPES: [&str; 22] = [
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32",
    "f64", "bool", "char", "Option", "Result", "String", "str", "Vec", "HashMap",
];

const KNOWN_VALUES: [&str; 6] = ["Some", "None", "true", "false", "Ok", "Err"];

pub struct RustSyntaxHighlighter {
    highlights: HashMap<LineIdx, Vec<Annotation>>,
    ml_comment_balance: usize,
    in_ml_string: bool,
}

impl RustSyntaxHighlighter {
    pub fn new() -> Self {
        RustSyntaxHighlighter {
            highlights: HashMap::new(),
            ml_comment_balance: 0,
            in_ml_string: false,
        }
    }

    fn is_numeric_literal(word: &str) -> bool {
        if word.len() < 3 {
            return false;
        }

        let mut chars = word.chars();

        if chars.next() != Some('0') {
            return false;
        }

        let base = match chars.next() {
            Some('o' | '0') => 8,
            Some('b' | 'B') => 2,
            Some('x' | 'X') => 16,
            _ => return false,
        };

        chars.all(|x| x.is_digit(base))
    }

    fn is_a_type(word: &str) -> bool {
        TYPES.contains(&word)
    }

    fn is_a_keyword(word: &str) -> bool {
        KEYWORDS.contains(&word)
    }

    fn is_a_known_value(word: &str) -> bool {
        KNOWN_VALUES.contains(&word)
    }

    fn is_a_number(word: &str) -> bool {
        if word.is_empty() {
            return false;
        }

        if Self::is_numeric_literal(word) {
            return true;
        }

        let mut chars = word.chars();

        if let Some(first_character) = chars.next() {
            if !first_character.is_ascii_digit() {
                return false;
            }
        }

        let mut seen_dot = false;
        let mut prev_was_digit = true;
        let mut seen_e = false;

        for ch in chars {
            match ch {
                '0'..='9' => {
                    prev_was_digit = true;
                }

                '_' => {
                    if !prev_was_digit {
                        return false;
                    }
                    prev_was_digit = false;
                }

                'e' | 'E' => {
                    if seen_e || !prev_was_digit {
                        return false;
                    }
                    prev_was_digit = false;
                    seen_e = true;
                }

                '.' => {
                    if seen_dot || !prev_was_digit || seen_e {
                        return false;
                    }
                    seen_dot = true;
                    prev_was_digit = false;
                }

                _ => {
                    return false;
                }
            };
        }
        prev_was_digit
    }

    fn annotate_next_word<T>(
        string: &str,
        annotation_type: AnnotationType,
        validator: T,
    ) -> Option<Annotation>
    where
        T: Fn(&str) -> bool,
    {
        if let Some(word) = string.split_word_bounds().next() {
            if validator(word) {
                return Some(Annotation {
                    start_byte: 0,
                    end_byte: word.len(),
                    annotation_type,
                });
            }
        }

        None
    }

    fn annotate_number(string: &str) -> Option<Annotation> {
        Self::annotate_next_word(string, AnnotationType::Number, Self::is_a_number)
    }

    fn annotate_type(string: &str) -> Option<Annotation> {
        Self::annotate_next_word(string, AnnotationType::Type, Self::is_a_type)
    }

    fn annotate_known_value(string: &str) -> Option<Annotation> {
        Self::annotate_next_word(string, AnnotationType::KnownValue, Self::is_a_known_value)
    }

    fn annotate_keyword(string: &str) -> Option<Annotation> {
        Self::annotate_next_word(string, AnnotationType::KeyWord, Self::is_a_keyword)
    }

    fn annotate_char(string: &str) -> Option<Annotation> {
        let mut iter = string.split_word_bound_indices().peekable();
        if let Some((_, "\'")) = iter.next() {
            if let Some((_, "\\")) = iter.peek() {
                iter.next(); //Skip the escape character
            }
            iter.next(); //Skip until the closing quote
            if let Some((idx, "\'")) = iter.next() {
                return Some(Annotation {
                    annotation_type: AnnotationType::Char,
                    start_byte: 0,
                    end_byte: idx.saturating_add(1), //Include the closing quote in the annotation
                });
            }
        }
        None
    }

    fn annotate_lifetime(string: &str) -> Option<Annotation> {
        let mut iter = string.split_word_bound_indices().peekable();
        if let Some((_, "\'")) = iter.next() {
            if let Some((idx, next_word)) = iter.next() {
                return Some(Annotation {
                    annotation_type: AnnotationType::Lifetime,
                    start_byte: 0,
                    end_byte: idx.saturating_add(next_word.len()), //Include the closing quote in the annotation
                });
            }
        }
        None
    }

    fn annotate_single_line_comment(string: &str) -> Option<Annotation> {
        if string.starts_with("//") {
            return Some(Annotation {
                start_byte: 0,
                end_byte: string.len(),
                annotation_type: AnnotationType::Comment,
            });
        }

        None
    }

    fn annotate_multi_line_comment(&mut self, string: &str) -> Option<Annotation> {
        let mut chars = string.char_indices().peekable();
        while let Some((_, char)) = chars.next() {
            if char == '/' {
                //check for an ml comment opener
                if let Some((_, '*')) = chars.peek() {
                    self.ml_comment_balance = self.ml_comment_balance.saturating_add(1);
                    chars.next();
                }
            } else if self.ml_comment_balance == 0 {
                return None; // We saw no opener, and we are not currently in a ML comment, returning None
            } else if char == '*' {
                if let Some((idx, '/')) = chars.peek() {
                    self.ml_comment_balance = self.ml_comment_balance.saturating_sub(1);
                    if self.ml_comment_balance == 0 {
                        return Some(Annotation {
                            annotation_type: AnnotationType::Comment,
                            start_byte: 0,
                            end_byte: idx.saturating_add(1),
                        });
                    }
                    chars.next();
                }
            }
        }
        (self.ml_comment_balance > 0).then_some(Annotation {
            annotation_type: AnnotationType::Comment,
            start_byte: 0,
            end_byte: string.len(),
        })
    }

    fn annotate_string(&mut self, string: &str) -> Option<Annotation> {
        let mut chars = string.char_indices();
        while let Some((idx, char)) = chars.next() {
            if char == '\\' && self.in_ml_string {
                chars.next(); // Skip the escape character.
                continue;
            }

            if char == '"' {
                if self.in_ml_string {
                    self.in_ml_string = false;
                    return Some(Annotation {
                        annotation_type: AnnotationType::String,
                        start_byte: 0,
                        end_byte: idx.saturating_add(1),
                    });
                }
                self.in_ml_string = true;
            }

            if !self.in_ml_string {
                return None;
            }
        }

        self.in_ml_string.then_some(Annotation {
            annotation_type: AnnotationType::String,
            start_byte: 0,
            end_byte: string.len(),
        })
    }

    fn initial_annotation(&mut self, line: &Line) -> Option<Annotation> {
        if self.in_ml_string {
            self.annotate_string(line)
        } else if self.ml_comment_balance > 0 {
            self.annotate_multi_line_comment(line)
        } else {
            None
        }
    }

    fn annotate_remainder(&mut self, remainder: &str) -> Option<Annotation> {
        self.annotate_multi_line_comment(remainder)
            .or_else(|| Self::annotate_single_line_comment(remainder))
            .or_else(|| Self::annotate_lifetime(remainder))
            .or_else(|| Self::annotate_char(remainder))
            .or_else(|| Self::annotate_number(remainder))
            .or_else(|| Self::annotate_type(remainder))
            .or_else(|| Self::annotate_known_value(remainder))
            .or_else(|| Self::annotate_keyword(remainder))
    }
}

impl SyntaxHighlighter for RustSyntaxHighlighter {
    fn highlight(&mut self, idx: LineIdx, line: &Line) {
        let mut result = Vec::new();

        let mut iterator = line.split_word_bound_indices().peekable();
        if let Some(annotation) = self.initial_annotation(line) {
            //handle dangling multi line annotations (i.e. ML comments or strings)

            result.push(annotation);
            // Skip over any subsequent word which has already been annotated in this step
            while let Some(&(next_idx, _)) = iterator.peek() {
                if next_idx >= annotation.end_byte {
                    break;
                }
                iterator.next();
            }
        }
        while let Some((start_idx, _)) = iterator.next() {
            let remainder = &line[start_idx..];
            if let Some(mut annotation) = self.annotate_remainder(remainder) {
                annotation.shift(start_idx);
                result.push(annotation);
                // Skip over any subsequent word which has already been annotated in this step
                while let Some(&(next_idx, _)) = iterator.peek() {
                    if next_idx >= annotation.end_byte {
                        break;
                    }
                    iterator.next();
                }
            };
        }
        self.highlights.insert(idx, result);
    }

    fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>> {
        self.highlights.get(&idx)
    }
}
