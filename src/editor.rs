use std::io::{stdin, stdout, Error, Stdout, Write};
use termion::color;

use termion::cursor::DetectCursorPos;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::buffer::Buffer;

#[derive(Default)]
enum EditorState {
    #[default]
    Init,
    Buffer,
    TakingFileInput,
}

#[derive(Default)]
pub struct Editor {
    buffer_index: usize,
    buffers: Vec<Buffer>,
    filename: String,
    exit: bool,
    mode: EditorState,
    error_message: String,
}

impl Editor {
    pub fn run(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        write!(
            stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )
        .unwrap();

        loop {
            self.render(&mut stdout);

            if self.exit {
                print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
                break;
            }
            self.process_input_event();
        }
    }

    fn read_next_event(&self) -> Result<Event, Error> {
        loop {
            if let Some(event) = stdin().events().next() {
                return event;
            }
        }
    }

    fn process_input_event(&mut self) {
        let event = self.read_next_event().unwrap();

        match self.mode {
            EditorState::Init => match event {
                Event::Key(Key::Esc) => {
                    print!("{}", termion::clear::All);
                    self.exit = true;
                }
                Event::Key(Key::Ctrl('n')) => self.mode = EditorState::TakingFileInput,
                _ => {}
            },
            EditorState::Buffer => {
                let buffer = &mut self.buffers[self.buffer_index];

                match event {
                    Event::Key(Key::Esc) => {
                        print!("{}", termion::clear::All);
                        self.drop_buffer();
                    }
                    Event::Key(Key::Char(char)) => {
                        buffer.write(char);
                    }
                    Event::Key(Key::Ctrl('s')) => {
                        buffer.save().unwrap();
                    }
                    Event::Key(Key::Ctrl('n')) => self.mode = EditorState::TakingFileInput,
                    Event::Key(Key::Ctrl('r')) => {
                        self.cycle_buffer();
                    }
                    Event::Key(Key::Ctrl('w')) => {
                        self.buffers[self.buffer_index].toggle_wrapping();
                    }
                    Event::Key(Key::Backspace) => {
                        buffer.delete();
                    }
                    Event::Key(Key::Up) => {
                        buffer.up();
                    }
                    Event::Key(Key::Down) => buffer.down(),
                    Event::Key(Key::Left) => {
                        buffer.left();
                    }
                    Event::Key(Key::Right) => {
                        buffer.right();
                    }
                    _ => {}
                }
            }
            EditorState::TakingFileInput => match event {
                Event::Key(Key::Esc) => {
                    print!("{}", termion::clear::All);
                    if !self.buffers.is_empty() {
                        self.mode = EditorState::Buffer;
                    } else {
                        self.mode = EditorState::Init;
                    }
                }
                Event::Key(Key::Char('\n')) => {
                    self.open_buffer();
                }
                Event::Key(Key::Char(char)) => {
                    self.filename.push(char);
                }
                Event::Key(Key::Backspace) => {
                    self.filename.pop();
                }
                _ => {}
            },
        }
    }

    fn render(&mut self, stdout: &mut RawTerminal<Stdout>) {
        match self.mode {
            EditorState::Init => {
                print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
                self.render_init_screen();
            }
            EditorState::Buffer => {
                self.render_buffer(stdout);
            }
            EditorState::TakingFileInput => {
                self.render_file_input();
            }
        }

        stdout.flush().unwrap();
    }

    fn render_buffer(&mut self, stdout: &mut RawTerminal<Stdout>) {
        let buffer = &mut self.buffers[self.buffer_index];

        let (row, col) = stdout.cursor_pos().unwrap();

        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        buffer.render();
        let (y, x) = termion::terminal_size().unwrap();

        let row_col_string = &*format!("{}:{}", buffer.buffer_row(), buffer.buffer_col(),);

        let empty_white_space =
            " ".repeat(y as usize - row_col_string.len() - buffer.filename().len());

        print!("{}{}", color::Bg(color::White), color::Fg(color::Black));
        print!(
            "{}{}{}",
            termion::cursor::Goto(1, x),
            buffer.filename(),
            empty_white_space
        );

        print!(
            "{}{}",
            termion::cursor::Goto(y + 1 - row_col_string.len() as u16, x),
            row_col_string
        );

        print!("{}{}", color::Bg(color::Reset), color::Fg(color::Reset));
        print!("{}", termion::cursor::Goto(row, col));
    }

    fn render_file_input(&mut self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        print!("{}\r\n", self.error_message);
        print!("Enter filename below, press Esc to go back\r\n");
        print!(
            "filename (relative path or absolute path): {}",
            self.filename
        );

        let (y, x) = termion::terminal_size().unwrap();

        let editor_name = "text-editor";
        let empty_white_space = " ".repeat(y as usize - editor_name.len());

        print!("{}{}", color::Bg(color::White), color::Fg(color::Black));
        print!(
            "{}{}{}",
            termion::cursor::Goto(1, x),
            editor_name,
            empty_white_space
        );

        print!("{}{}", color::Bg(color::Reset), color::Fg(color::Reset));
    }

    fn render_init_screen(&self) {
        print!("A simple text editor written in rust\n\r");
        print!("~\n\r");
        print!("Ctrl + N : open new file to edit\n\r");
        print!("Ctrl + R : to rotate between open files\n\r");
        print!("Ctrl + S : Save current file\n\r");
        print!("~\n\r");
        print!("Ctrl + W : Toggle text wrapping\n\r");
        print!("Arrows   : cursor movement\n\r");
        print!("Backspace: erase character\n\r");
        print!("~\n\r");
        print!("Esc      : Exit\n\r");

        let (y, x) = termion::terminal_size().unwrap();

        let editor_name = "text-editor";
        let empty_white_space = " ".repeat(y as usize - editor_name.len());

        print!("{}{}", color::Bg(color::White), color::Fg(color::Black));
        print!(
            "{}{}{}",
            termion::cursor::Goto(1, x),
            editor_name,
            empty_white_space
        );

        print!("{}{}", color::Bg(color::Reset), color::Fg(color::Reset));
    }

    fn open_buffer(&mut self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        if let Ok(buffer) = Buffer::new(self.filename.as_str()) {
            self.buffers.push(buffer);
            self.buffer_index = self.buffers.len() - 1;
            self.mode = EditorState::Buffer;
            self.filename = String::new();
            self.error_message = String::new();
        } else {
            self.error_message = format!("file {} not found, enter correct path", self.filename);
            self.filename = String::new();
        }
    }

    fn drop_buffer(&mut self) {
        self.buffers.remove(self.buffer_index);
        self.cycle_buffer();
        if self.buffers.is_empty() {
            self.mode = EditorState::Init;
        }
    }

    fn cycle_buffer(&mut self) {
        self.buffer_index += 1;
        if self.buffer_index >= self.buffers.len() {
            self.buffer_index = 0;
        }
    }
}
