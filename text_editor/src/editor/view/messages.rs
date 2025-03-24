use crate::editor::terminal::TerminalSize;

use super::buffer::Buffer;

pub struct Message;

impl Message {
    pub fn build_welcome_message(size: TerminalSize, name: &str, version: &str) -> Buffer {
        let mut buffer = Buffer::default();

        let length_of_terminal = size.columns;

        let blank_line =
            String::from("|") + &" ".repeat(length_of_terminal.saturating_sub(2)) + "|";

        // Draw top bar
        buffer.push(&"=".repeat(length_of_terminal));

        // Draw sides
        for _ in 0..5 {
            buffer.push(&blank_line);
        }

        // Draw Name and version number
        let blank_space = (length_of_terminal - name.len() - version.len()).saturating_sub(3);

        let (padding_left, padding_right) = if blank_space % 2 == 0 {
            (&" ".repeat(blank_space / 2), &" ".repeat(blank_space / 2))
        } else {
            (
                &" ".repeat(blank_space / 2 + 1),
                &" ".repeat(blank_space / 2),
            )
        };

        let center_line = format!("|{padding_left}{name} {version}{padding_right}|");

        if center_line.len() >= size.columns {
            buffer.push(&center_line);
        } else {
            buffer.push(&blank_line);
        }

        // Draw sides
        for _ in 0..5 {
            buffer.push(&blank_line);
        }

        // Draw top bar
        buffer.push(&"=".repeat(length_of_terminal));

        buffer
    }
}
