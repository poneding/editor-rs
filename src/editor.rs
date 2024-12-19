use std::io::Error;
use std::process::exit;

use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        if let Err(err) = self.repl() {
            eprintln!("{}", err);
            exit(1);
        }
        println!("bye~\r\n");
    }

    pub fn repl(&self) -> Result<(), Error> {
        enable_raw_mode()?;
        loop {
            if let Event::Key(event) = read()? {
                if let KeyCode::Char('q') = event.code {
                    break;
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
