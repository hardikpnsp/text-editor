use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Lines, Read, Write};
use std::{env, io};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct Buffer {
    rows: Vec<String>,
}

impl Buffer {
    fn new(filename: &str) -> Self {
        let file = File::open(filename).expect("could not open file");
        let mut buffer: Vec<String> = vec![];

        for line in io::BufReader::new(file).lines() {
            let line = line.expect("failed reading line");
            buffer.push(line);
        }

        return Buffer { rows: buffer };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("file name not provided");

    let mut stdout = stdout().into_raw_mode().unwrap();
    let buffer: Buffer = Buffer::new(filename);

    write!(stdout, "{}", termion::clear::All).unwrap();
    for line in buffer.rows {
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
    }
}
