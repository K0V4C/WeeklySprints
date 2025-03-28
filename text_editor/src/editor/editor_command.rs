use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use super::terminal::TerminalSize;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
}

pub enum EditorCommand {
    Move(Direction),
    Resize(TerminalSize),
    Input(char),
    Delete,
    Backspace,
    Tab,
    Enter,
    Quit,
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),

                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),

                (KeyCode::Char(chr), _) => Ok(Self::Input(chr)),

                (KeyCode::Backspace, _) => Ok(Self::Backspace),
                (KeyCode::Delete, _) => Ok(Self::Delete),

                (KeyCode::Tab, _) => Ok(Self::Tab),
                (KeyCode::Enter, _) => Ok(Self::Enter),

                _ => Err(format!("Key code not supported: {code:?}")),
            },

            Event::Resize(width_u16, heigth_16) => {
                let height = heigth_16 as usize;
                let width = width_u16 as usize;

                Ok(Self::Resize(TerminalSize {
                    columns: width,
                    rows: height,
                }))
            }

            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
