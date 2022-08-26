use std::io::{stdin, stdout, Read, Write};
use std::{env, fs};
use termion::color;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("file name not provided");
    let buffer = fs::read_to_string(filename).expect("could not read the file content");

    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}", termion::clear::All).unwrap();
    write!(stdout, "{}", buffer).unwrap();
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    stdout.flush().unwrap();

    let stdin = stdin();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Esc) => break,
            _ => {}
        }
    }
}
