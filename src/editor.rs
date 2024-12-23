use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io::Error;

mod terminal;
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap(); // why after terminate? because we need to disable raw mode before exiting
    }

    #[allow(unused)]
    pub fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Event::Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End => {
                    self.move_point(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;
        match key_code {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => y = core::cmp::min(height.saturating_sub(1), y.saturating_add(1)),
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => x = core::cmp::min(width.saturating_sub(1), x.saturating_add(1)),
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = height.saturating_sub(1),
            KeyCode::Home => x = 0,
            KeyCode::End => x = width.saturating_sub(1),
            _ => (),
        }
        self.location = Location { x, y };
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        // Terminal::hide_cursor()?;
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            // Terminal::move_cursor_to(Position { x: 0, y: 0 })?; // Move the cursor to the top-left corner.
            Terminal::move_caret_to(Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }

        // Terminal::show_cursor()?;
        Terminal::show_caret()?;
        Terminal::execute()?;

        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for cur_row in 0..height {
            Terminal::clear_line()?;
            #[allow(clippy::integer_division)]
            if cur_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_rows()?;
            }
            if cur_row < height.saturating_sub(1) {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_message.len();

        #[allow(clippy::integer_division)] // integer_division 表示整数除法
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(welcome_message)
    }

    fn draw_empty_rows() -> Result<(), Error> {
        Terminal::print("~")
    }
}
