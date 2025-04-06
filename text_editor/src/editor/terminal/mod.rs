use std::io::{Error, Write, stdout};

mod attribute;

use crossterm::{
    Command,
    cursor::{DisableBlinking, EnableBlinking, Hide, MoveTo, SavePosition, Show},
    execute, queue,
    style::{Attribute, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{
        Clear, ClearType, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen, SetTitle, disable_raw_mode, enable_raw_mode,
    },
};

use super::{annotated_string::AnnotatedString, caret_position::CaretPosition, size::Size};

pub struct Terminal;

impl Terminal {
    pub fn init() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::disable_line_wrap()?;
        Self::disable_blinking()?;
        Self::clear_screen()?;
        Self::draw()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        Self::exit_alternate_screen()?;
        Self::show_caret()?;
        Self::enable_line_wrap()?;
        Self::enable_blinking()?;
        Self::draw()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        Self::move_caret_to(CaretPosition { column: 0, row: 0 })?;
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    pub fn exit_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn disable_line_wrap() -> Result<(), Error> {
        Self::queue_command(DisableLineWrap)?;
        Ok(())
    }

    pub fn enable_line_wrap() -> Result<(), Error> {
        Self::queue_command(EnableLineWrap)?;
        Ok(())
    }

    pub fn set_title(title: &str) -> Result<(), Error> {
        Self::queue_command(SetTitle(title))?;
        Ok(())
    }

    pub fn size() -> Result<Size, std::io::Error> {
        let size = crossterm::terminal::size()?;

        Ok(Size {
            columns: size.0 as usize,
            rows: size.1 as usize,
        })
    }

    pub fn _get_caret_position() -> Result<CaretPosition, std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, SavePosition)?;

        let position = crossterm::cursor::position()?;

        Ok(CaretPosition {
            column: position.0 as usize,
            row: position.1 as usize,
        })
    }

    pub fn disable_blinking() -> Result<(), std::io::Error> {
        Self::queue_command(DisableBlinking)?;
        Ok(())
    }
    pub fn enable_blinking() -> Result<(), std::io::Error> {
        Self::queue_command(EnableBlinking)?;
        Ok(())
    }

    pub fn hide_caret() -> Result<(), std::io::Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), std::io::Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), std::io::Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn draw() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }

    /// There could be and edge case on systems where u16 < usize
    /// In that case check this piece of code
    /// Maybe need to refactor it
    pub fn move_caret_to(caret_positon: CaretPosition) -> Result<(), std::io::Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        let x = caret_positon.column as u16;
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        let y = caret_positon.row as u16;
        Self::queue_command(MoveTo(x, y))?;
        Ok(())
    }

    pub fn print_annoted_line(
        row: usize,
        annotated_string: AnnotatedString,
    ) -> Result<(), std::io::Error> {
        Self::move_caret_to(CaretPosition { column: 0, row })?;
        Self::clear_line()?;
        for part in annotated_string.into_iter() {
            if let Some(a_type) = part.annotaion_type {
                let atr = a_type.into();
                Self::set_attribute(atr)?;
            }
            Self::print(&format!("{}", part.string))?;
            Self::reset_color()?;
        }
        Ok(())
    }

    fn reset_color() -> Result<(), std::io::Error> {
        Self::queue_command(ResetColor)?;
        Ok(())
    }

    fn set_attribute(attribute: attribute::Attribute) -> Result<(), std::io::Error> {
        if let Some(foreground) = attribute.foreground {
            Self::queue_command(SetForegroundColor(foreground))?;
        }

        if let Some(background) = attribute.background {
            Self::queue_command(SetBackgroundColor(background))?;
        }
        Ok(())
    }

    pub fn print_row_with_attribute(
        row: usize,
        attr: Attribute,
        line_text: &str,
    ) -> Result<(), std::io::Error> {
        Self::move_caret_to(CaretPosition { column: 0, row })?;
        Self::clear_line()?;
        Self::print(&format!("{}{}{}", attr, line_text, Attribute::Reset))?;
        Ok(())
    }

    pub fn print_row(row: usize, line_text: &str) -> Result<(), std::io::Error> {
        Self::move_caret_to(CaretPosition { column: 0, row })?;
        Self::clear_line()?;
        Self::print(line_text)?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        queue!(stdout, command)?;
        Ok(())
    }
}
