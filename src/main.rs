use crossterm::event::{Event, KeyCOde, KeyEvent};
use crossterm::terminal;
use std::time::Duration;

use std::io;
use std::io::Read;

struct CleanUp;


impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().exptect("Could not disable raw mode")
    }
}

fn main() {
    let _cleanu_up = CleanuUp;
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");
    loop {
        if let Event::Key(event) = event::read().expect("Failed to read line") {
            match event {
                KeyEvent {
                    code: KeyCode::Char('q),
                    modifiers: event::KeyModifiers::NONE,
                } => break,
                _ => {
                }
            }
            println!("{:?}\r", event);
        };
    }

    let mut buf = [0; 1];
    /*modify*/
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {
        let character = buf[0] as char;
        if character.is_control() {
            println!("{}\r, character as u8)
            } else {
                println!("{}\r", character)
                }
        }
}

