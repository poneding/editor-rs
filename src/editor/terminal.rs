use std::{
    fmt::Display,
    io::{stdout, Error, Write},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    Command,
};

#[derive(Clone, Copy, Default)]
pub(crate) struct Size {
    pub(crate) height: usize,
    pub(crate) width: usize,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct Position {
    pub(crate) col: usize,
    pub(crate) row: usize,
}

pub(crate) struct Terminal {}

impl Terminal {
    pub(crate) fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()
    }

    pub(crate) fn initialize() -> Result<(), Error> {
        enable_raw_mode()?; // Enable raw mode.
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::execute()
    }

    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(crossterm::terminal::EnterAlternateScreen)
    }

    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(crossterm::terminal::LeaveAlternateScreen)
    }

    pub(crate) fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))
    }

    pub(crate) fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub(crate) fn move_caret_to(pos: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(pos.col as u16, pos.row as u16))
    }

    pub(crate) fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    pub fn print_row(row: usize, line_text: &str) -> Result<(), Error> {
        Self::move_caret_to(Position { col: 0, row })?;
        Self::clear_line()?;
        Self::print(line_text)
    }

    pub(crate) fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)
    }

    pub(crate) fn print<T: Display>(t: T) -> Result<(), Error> {
        Self::queue_command(Print(t))
    }

    /// Get the size of the terminal.
    pub(crate) fn size() -> Result<Size, Error> {
        let (width, height) = size()?;

        #[allow(clippy::as_conversions)]
        let height = height as usize;
        #[allow(clippy::as_conversions)]
        let width = width as usize;
        Ok(Size { height, width })
    }

    /// Execute the queued commands.
    pub(crate) fn execute() -> Result<(), Error> {
        stdout().flush()
    }

    /// Queue the specified command.
    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)
    }
}
