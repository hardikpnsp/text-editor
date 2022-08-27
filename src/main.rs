use std::env;
use std::io::{stdin, stdout, Stdout, Write};
use termion::cursor::DetectCursorPos;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use text_editor::buffer::Buffer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("file name not provided");

    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut buffer: Buffer = Buffer::new(filename);

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    render(&mut stdout, &mut buffer);

    let stdin = stdin();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Esc) => {
                write!(stdout, "{}", termion::clear::All).unwrap();
                return;
            },
            Event::Key(Key::Char(char)) => {
                buffer.write(char);
            },
            Event::Key(Key::Ctrl('s')) => {
                buffer.save(filename).unwrap();
            },
            Event::Key(Key::Backspace) => {
                buffer.delete();
            },
            Event::Key(Key::Up) => {
                buffer.cursor.up();
            },
            Event::Key(Key::Down) => {
                buffer.cursor.down();
            },
            Event::Key(Key::Left) => {
                buffer.cursor.left();
            },
            Event::Key(Key::Right) => {
                buffer.cursor.right();
            }
            _ => {}
        }

        render(&mut stdout, &mut buffer);
    }
}

fn render(stdout: &mut RawTerminal<Stdout>, buffer: &mut Buffer) {
    let (row, col) = stdout.cursor_pos().unwrap();
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    for line in buffer.rows() {
        write!(stdout, "{}\r\n", line).unwrap();
    }
    // restor cursor position
    write!(stdout, "{}", termion::cursor::Goto(row, col)).unwrap();
    stdout.flush().unwrap();
}
