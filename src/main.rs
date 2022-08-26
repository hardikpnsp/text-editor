use std::io::{stdin, stdout, Read, Write, BufRead, BufReader, Lines};
use std::{env, io};
use std::fs::File;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("file name not provided");

    let file = File::open(filename).expect(&format!("could not open file: {}", filename));

    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut buffer: Vec<String> = vec![];

    for line in io::BufReader::new(file).lines() {
        let line = line.expect("failed reading line");
        buffer.push(line);
    }

    write!(stdout, "{}", termion::clear::All).unwrap();
    for line in buffer {
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
