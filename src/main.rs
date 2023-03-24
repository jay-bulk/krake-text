use crossterm::event::{Event, KeyCOde, KeyEvent};
use crossterm::{event, terminal};
use std::time::Duration;

use std::io;
use std::io::Read;

struct CleanUp;
struct Reader;
struct Editor {
    reader: Reader,
}


impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().exptect("Could not disable raw mode")
    }
}


impl Reader {
    fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}

impl Editor {
    fn new() -> Self {
        Self { reader: Reader }
    }

    fn process_keypress(&self) -> crossterm:Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
            } => return Ok(false),
            _ => {}
        }
        Ok(true)
    }

    fn run(&self) -> crossterm::Result<bool> {
        self.process_keypress()
    }
}


fn main() {
    let _cleanu_up = CleanuUp;
    terminal::enable_raw_mode()?;
    loop {
            if event::poll(Duration::from_millis(1000))? {
            if let Event::Key(event) = event::read()? {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('q),
                        modifiers: event::KeyModifiers::CONTROL,
                    } => break,
                    _ => {
                        //todo
                    }
                }
                println!("{:?}\r", event);
            };
        } else {
            println!("No inpuet yet\r");
            }
    }
    Ok(())
}

