use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io::Error;

use crate::terminal::{Position, Size, Terminal};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

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

    fn evaluate_event(&mut self, event: &Event) {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
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
                // KeyCode::Char(c) => print!("{}", c),
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("bye.")?;
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
            Terminal::print("~")?;
            if cur_row < height - 1 {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }
}
