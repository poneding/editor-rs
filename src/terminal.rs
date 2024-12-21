use std::io::{stdout, Error, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};

#[derive(Clone, Copy)]
pub(crate) struct Size {
    pub(crate) height: u16,
    pub(crate) width: u16,
}

#[derive(Clone, Copy)]
pub(crate) struct Position {
    pub(crate) x: u16,
    pub(crate) y: u16,
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
        // print!(
        // "{}",
        // crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        // );
        execute!(stdout(), Clear(ClearType::All))
    }

    /// Clear the current line.
    pub(crate) fn clear_line() -> Result<(), Error> {
        execute!(stdout(), Clear(ClearType::CurrentLine))
    }

    /// Move the cursor to the specified position.
    pub(crate) fn move_cursor_to(pos: Position) -> Result<(), Error> {
        execute!(stdout(), MoveTo(pos.x, pos.y))
    }

    /// Get the size of the terminal.
    pub(crate) fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size { width, height })
    }

    pub(crate) fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)
    }

    pub(crate) fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)
    }

    pub(crate) fn print(s: &str) -> Result<(), Error> {
        queue!(stdout(), Print(s))
    }

    pub(crate) fn execute() -> Result<(), Error> {
        stdout().flush()
    }
}
