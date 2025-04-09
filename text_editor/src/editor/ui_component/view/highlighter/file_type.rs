use std::fmt::Display;

#[derive(Clone, Copy, Default)]
pub enum FileType {
    Rust,
    Txt,
    #[default]
    None,
}

impl From<String> for FileType {
    fn from(value: String) -> Self {
        let last = value.split('.').last();

        match last {
            Some("rs") => FileType::Rust,
            Some("txt") => FileType::Txt,

            _ => FileType::None,
        }
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::Rust => write!(f, "Rust"),
            FileType::Txt => write!(f, "Text"),
            _ => write!(f, ""),
        }
    }
}
