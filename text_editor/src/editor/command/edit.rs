use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Clone, Copy)]
pub enum Edit {
    Input(char),
    Delete,
    Backspace,
    Tab,
    Enter,
}

impl TryFrom<KeyEvent> for Edit {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        match (code, modifiers) {
            (KeyCode::Backspace, KeyModifiers::NONE) => Ok(Self::Backspace),
            (KeyCode::Delete, KeyModifiers::NONE) => Ok(Self::Delete),

            (KeyCode::Tab, KeyModifiers::NONE) => Ok(Self::Tab),
            (KeyCode::Enter, KeyModifiers::NONE) => Ok(Self::Enter),

            (KeyCode::Char(chr), KeyModifiers::NONE | KeyModifiers::SHIFT) => Ok(Self::Input(chr)),
            _ => Err(format!("Movement key code not supported: {code:?}")),
        }
    }
}
