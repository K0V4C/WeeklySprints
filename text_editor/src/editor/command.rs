use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use super::terminal::TerminalSize;

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

#[derive(Clone, Copy)]
pub enum System {
    Resize(TerminalSize),
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

#[derive(Clone, Copy)]
pub enum Command {
    Move(Move),
    Edit(Edit),
    System(System),
}

impl TryFrom<Event> for Command {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(key_event) => System::try_from(key_event)
                .map(Command::System)
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| Edit::try_from(key_event).map(Command::Edit))
                .map_err(|_err| format!("Event not supported: {key_event:?}")),

            Event::Resize(width_u16, heigth_16) => {
                let height = heigth_16 as usize;
                let width = width_u16 as usize;

                Ok(Self::System(System::Resize(TerminalSize {
                    columns: width,
                    rows: height,
                })))
            }

            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
