use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io::Error;

use crate::terminal::{Position, Size, Terminal};

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
    loation: Location,
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
            self.evaluate_event(&event);
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
            // println!("{:?} {:?}", code, modifiers == &KeyModifiers::CONTROL);
            match code {
                // KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                //     self.should_quit = true
                // }
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    // ctrl+q
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
                    self.move_point(*code);
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.loation;
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("done.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?; // Move the cursor to the top-left corner.
        }

        Terminal::show_cursor()?;
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
