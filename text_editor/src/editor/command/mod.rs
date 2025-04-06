pub mod edit;
pub mod movement;
pub mod system;

use crossterm::event::Event;
use edit::Edit;
use movement::Move;
use system::System;

use crate::editor::size::Size;

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

                Ok(Self::System(System::Resize(Size {
                    columns: width,
                    rows: height,
                })))
            }

            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
