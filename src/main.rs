use std::env;
use std::io::{stdin, stdout, Stdout, Write};

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use buffer::Buffer;

mod buffer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("file name not provided");

    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut buffer: Buffer = Buffer::new(filename);

    render(&mut stdout, &mut buffer);

    let stdin = stdin();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Esc) => break,
            Event::Key(Key::Char(char)) => {
                buffer.write(char);
            },
            Event::Key(Key::Ctrl('s')) => {
                buffer.save(filename);
            },
            Event::Key(Key::Backspace) => {
                buffer.delete();
            },
            _ => {}
        }

        render(&mut stdout, &mut buffer);
    }
}

fn render(stdout: &mut RawTerminal<Stdout>, buffer: &mut Buffer) {
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    for line in buffer.rows() {
        write!(stdout, "{}\r\n", line).unwrap();
    }
    stdout.flush().unwrap();
}
