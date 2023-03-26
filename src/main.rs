use crossterm::event::*;
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, execute, terminal};
use std::io::stdout;
use std::time::Duration;

struct CleanUp;
struct Reader;
struct Output {
    win_size: (usize, usize),
}
struct Editor {
    reader: Reader,
    output: Output,
}

struct EditorContents {
    content: String,
}

impl EditorContents {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}

impl io::Write for EditorContents {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }
    
    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        Output::clear_screen().expect("Error");
    }
}

impl Output {
    fn new() -> Self {

        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self { win_size }
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0,0))
    }

    fn draw_rows(&self) {
        let screen_rows = self.win_size.1;
        for i in 0..screen_rows {
            print!("~");
            if i < screen_rows - 1 {
                println!("\r")
            }
        }
        stdout().flush();
    }

    fn refresh_screen(&self) -> crossterm::Result<()> {
        Self::clear_screen()?;
        self.draw_rows();
        execute!(stdout(), cursor::MoveTo(0,0))
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
            output: Output::new(),
        }
    }

    fn process_keypress(&self) -> crossterm::Result<bool> {
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
    let _clean_up = CleanUp;
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

