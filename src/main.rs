mod buffer;

use std::io::{stdin, stdout, Write};
use std::env;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use buffer::Buffer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("file name not provided");

    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut buffer: Buffer = Buffer::new(filename);

    write!(stdout, "{}", termion::clear::All).unwrap();
    for line in buffer.rows() {
        write!(stdout, "{}\r\n", line).unwrap();
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    stdout.flush().unwrap();

    let stdin = stdin();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Esc) => break,
            _ => {}
        }
        for line in buffer.rows() {
            write!(stdout, "{}\r\n", line).unwrap();
        }
        stdout.flush().unwrap();
    }
}
