use crossterm::style::Color;

use crate::editor::annotated_string::annotation_type::AnnotationType;

pub struct Attribute {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl From<AnnotationType> for Attribute {
    fn from(annotation_type: AnnotationType) -> Self {
        match annotation_type {
            AnnotationType::Match => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 100,
                    g: 100,
                    b: 100,
                }),
            },
            AnnotationType::SelectedMatch => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 255,
                    g: 251,
                    b: 0,
                }),
            },
            AnnotationType::Digit => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 99,
                    b: 71,
                }),
                background: None,
            },
        }
    }
}
