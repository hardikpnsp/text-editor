use std::io::{stdin, stdout, Stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use crate::buffer::Buffer;

pub struct Editor<'a> {
    buffer: Buffer,
    filename: &'a str
}

impl<'a> Editor <'a> {
    pub fn new(filename: &'a str) -> Self {
        Editor {
            buffer: Buffer::new(filename),
            filename
        }
    }
    pub fn run(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
        self.render(&mut stdout);

        let stdin = stdin();

        for c in stdin.events() {
            let evt = c.unwrap();
            match evt {
                Event::Key(Key::Esc) => {
                    write!(stdout, "{}", termion::clear::All).unwrap();
                    return;
                },
                Event::Key(Key::Char(char)) => {
                    self.buffer.write(char);
                },
                Event::Key(Key::Ctrl('s')) => {
                    self.buffer.save(self.filename).unwrap();
                },
                Event::Key(Key::Backspace) => {
                    self.buffer.delete();
                },
                Event::Key(Key::Up) => {
                    self.buffer.cursor.up();
                },
                Event::Key(Key::Down) => {
                    self.buffer.cursor.down();
                },
                Event::Key(Key::Left) => {
                    self.buffer.cursor.left();
                },
                Event::Key(Key::Right) => {
                    self.buffer.cursor.right();
                }
                _ => {}
            }

            self.render(&mut stdout);
        }
    }

    fn render(&self, stdout: &mut RawTerminal<Stdout>) {
        let (row, col) = stdout.cursor_pos().unwrap();
        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
        for line in self.buffer.rows() {
            write!(stdout, "{}\r\n", line).unwrap();
        }

        write!(stdout, "{}", termion::cursor::Goto(row, col)).unwrap();
        stdout.flush().unwrap();
    }
}