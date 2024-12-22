use std::{
    fmt::Display,
    io::{stdout, Error, Write},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
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

#[derive(Clone, Copy)]
pub(crate) struct Position {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

pub(crate) struct Terminal {}

impl Terminal {
    /// Initialize the editor.
    pub(crate) fn initialize() -> Result<(), Error> {
        enable_raw_mode()?; // Enable raw mode.
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
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

    /// Move the cursor to the specified position.
    pub(crate) fn move_cursor_to(pos: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(pos.x as u16, pos.y as u16))
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

    /// Hide the cursor.
    pub(crate) fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    /// Show the cursor.
    pub(crate) fn show_cursor() -> Result<(), Error> {
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
