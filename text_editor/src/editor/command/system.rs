use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::editor::size::Size;


#[derive(Clone, Copy)]
pub enum System {
    Resize(Size),
    Save,
    Quit,
    Abort,
    Search,
}

impl TryFrom<KeyEvent> for System {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        match (code, modifiers) {
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => Ok(Self::Save),
            (KeyCode::Char('f'), KeyModifiers::CONTROL) => Ok(Self::Search),
            (KeyCode::Esc, KeyModifiers::NONE) => Ok(Self::Abort),

            _ => Err(format!("Movement key code not supported: {code:?}")),
        }
    }
}
