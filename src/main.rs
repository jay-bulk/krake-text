use crossterm::event::*
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, execute, terminal};
use std::io::stdout;
use std::time::Duration;

struct CleanUp;
struct Reader;
struct Output;
struct Editor {
    reader: Reader,
    output: Output,
}


impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().exptect("Could not disable raw mode")
    }
}

impl Output {
    fn new() -> Self {
        Self 
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))
    }

    fn refresh_screen(&self) -> crossterm::Result<()> {
        Self::clean_screen()
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
        Self { 
            reader: Reader,
            output: Outpit::new(),
        }
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
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}


fn main() -> crossterm::Result<()> {
    let _cleanu_up = CleanuUp;
    terminal::enable_raw_mode()?;

    let editor = Editor::new();

    while editor.run()? {}

    Ok(())

    //loop {
    //        if event::poll(Duration::from_millis(1000))? {
    //        if let Event::Key(event) = event::read()? {
    //            match event {
    //                KeyEvent {
    //                    code: KeyCode::Char('q),
    //                    modifiers: event::KeyModifiers::CONTROL,
    //                } => break,
    //                _ => {
    //                    //todo
    //                }
    //            }
    //            println!("{:?}\r", event);
    //        };
    //    } else {
    //        println!("No inpuet yet\r");
    //        }
    //}
    //Ok(())
}

