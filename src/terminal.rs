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

#[allow(dead_code)]
#[derive(Clone, Copy)]
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
    /// Initialize the editor.
    pub(crate) fn initialize() -> Result<(), Error> {
        enable_raw_mode()?; // Enable raw mode.
        Self::clear_screen()?;
        // Self::move_cursor_to(Position { col: 0, row: 0 })?;
        Self::execute()?; // Hide the cursor.
        Ok(())
    }

    /// Terminate the editor.
    pub(crate) fn terminate() -> Result<(), Error> {
        disable_raw_mode()?; // Disable raw mode.
        Ok(())
    }

    /// Clear the screen.
    pub(crate) fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))
    }

    /// Clear the current line.
    pub(crate) fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    /// Move the caret to the specified position.
    pub(crate) fn move_caret_to(pos: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(pos.col as u16, pos.row as u16))
    }

    /// Get the size of the terminal.
    pub(crate) fn size() -> Result<Size, Error> {
        let (width, height) = size()?;

        #[allow(clippy::as_conversion)]
        let height = height as usize;
        #[allow(clippy::as_conversions)]
        let width = width as usize;
        Ok(Size { width, height })
    }

    /// Hide the caret.
    pub(crate) fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    /// Show the caret.
    pub(crate) fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)
    }

    /// Print the specified value.
    pub(crate) fn print<T: Display>(t: T) -> Result<(), Error> {
        Self::queue_command(Print(t))
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
