use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Clone, Copy)]
pub enum Move {
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
}

impl TryFrom<KeyEvent> for Move {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers != KeyModifiers::NONE {
            return Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ));
        }

        match (code, modifiers) {
            (KeyCode::Up, _) => Ok(Self::Up),
            (KeyCode::Down, _) => Ok(Self::Down),
            (KeyCode::Left, _) => Ok(Self::Left),
            (KeyCode::Right, _) => Ok(Self::Right),
            (KeyCode::PageUp, _) => Ok(Self::PageUp),
            (KeyCode::PageDown, _) => Ok(Self::PageDown),
            (KeyCode::Home, _) => Ok(Self::Home),
            (KeyCode::End, _) => Ok(Self::End),
            _ => Err(format!("Movement key code not supported: {code:?}")),
        }
    }
}
