use crossterm::terminal;
use std::io;
use std::io::Read;

struct CleanUp;


impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().exptect("Could not disable raw mode")
    }
}

fn main() {
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");
    let mut buf = [0; 1];
    /*modify*/
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {}
    /*end*/
    painc!();
    //terminal::disable_raw_mode().expect("Could not turn off raw mode");
}