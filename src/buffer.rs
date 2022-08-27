use std::fs::File;
use std::io;
use std::io::{BufRead, LineWriter, Write};

use crate::cursor::Cursor;

struct Line {
    value: String,
}

impl Line {
    pub fn from(value: String) -> Self {
        Line { value }
    }

    pub fn render(&self) {
        print!("{}\r\n", self.value);
    }
}

pub struct Buffer {
    lines: Vec<Line>,
    pub cursor: Cursor,
}

impl Buffer {
    pub fn new(filename: &str) -> Self {
        let file = File::open(filename).expect("could not open file");
        let mut buffer: Vec<Line> = vec![];

        for line in io::BufReader::new(file).lines() {
            let line = line.expect("failed reading line");
            buffer.push(Line::from(line));
        }

        return Buffer {
            lines: buffer,
            cursor: Cursor::new(),
        };
    }

    pub fn write(&mut self, char: char) {
        self.adjust_cursor_boundary_before_edit();

        match char {
            '\n' => {
                let line = &self.lines[self.cursor.row()].value;
                if self.cursor.col() >= line.len() {
                    self.lines
                        .insert(self.cursor.row() + 1, Line::from(String::new()));
                    self.cursor.new_line();
                } else {
                    let cur = &line[..self.cursor.col()];
                    let next = &line[self.cursor.col()..];
                    let result = next.to_string();
                    self.lines[self.cursor.row()].value = cur.to_string();
                    self.lines.insert(self.cursor.row() + 1, Line::from(result));
                    self.cursor.new_line();
                }
            }
            _ => {
                let line = &self.lines[self.cursor.row()].value;
                if self.cursor.col() > line.len() {
                    self.lines[self.cursor.row()].value.push(char);
                } else {
                    let pre = &line[..self.cursor.col()];
                    let post = &line[self.cursor.col()..];
                    let mut result = pre.to_string();
                    result.push(char);
                    result.push_str(post);
                    self.lines[self.cursor.row()].value = result;
                    self.cursor.right();
                }
            }
        }
    }

    fn adjust_cursor_boundary_before_edit(&mut self) {
        if self.cursor.row() > self.lines.len() {
            self.cursor.goto(
                self.lines.len() - 1,
                self.lines[self.lines.len() - 1].value.len(),
            );
        }
        if self.cursor.col() > self.lines[self.cursor.row()].value.len() {
            self.cursor
                .goto(self.cursor.row(), self.lines[self.cursor.row()].value.len());
        }
    }

    pub fn delete(&mut self) {
        self.adjust_cursor_boundary_before_edit();

        if self.cursor.col() == 0 {
            if self.cursor.row() != 0 {
                let l = self.lines[self.cursor.row() - 1].value.len();

                let current_line = self.lines[self.cursor.row()].value.clone();
                self.lines[self.cursor.row() - 1]
                    .value
                    .push_str(&*current_line);

                self.cursor.delete_line(l);
                self.lines.remove(self.cursor.row() + 1);
            }
        } else {
            let line = &self.lines[self.cursor.row()].value;
            let cur = &line[..self.cursor.col()];
            let next = &line[self.cursor.col()..];
            let mut result = cur.to_string();
            result.pop();
            result.push_str(next);
            self.lines[self.cursor.row()].value = result;
            self.cursor.left();
        }
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename).expect("could not open file in write only mode");
        let mut file = LineWriter::new(file);

        for line in &self.lines {
            file.write_all(line.value.as_ref())?;
            file.write_all(b"\r\n")?;
        }

        file.flush()?;
        Ok(())
    }

    pub fn render(&self) {
        for line in &self.lines {
            line.render();
        }
    }
}
#[cfg(test)]
mod test {
    use std::fs::{remove_file, File};
    use std::io::{Read, Write};

    use termion::input::TermRead;

    use super::*;

    #[test]
    fn buffer_writes_saves_and_deletes() {
        let filename = "write_test_file.txt";
        let mut f = File::create(filename).unwrap();
        f.write_all(b", World").unwrap();

        let mut buffer = Buffer::new(filename);
        buffer.write('H');
        buffer.write('e');
        buffer.write('l');
        buffer.write('l');
        buffer.write('o');

        buffer.save(filename).unwrap();

        let mut file = File::open(filename).unwrap();
        assert_eq!(file.read_line().unwrap().unwrap(), "Hello, World");

        buffer.write('\n');
        buffer.save(filename).unwrap();

        let mut file = File::open(filename).unwrap();
        let mut result = String::new();
        file.read_to_string(&mut result).unwrap();
        assert_eq!(result, "Hello\r\n, World\r\n");

        buffer.delete();
        buffer.delete();
        buffer.delete();
        buffer.delete();
        buffer.delete();

        buffer.save(filename).unwrap();
        let mut file = File::open(filename).unwrap();
        assert_eq!(file.read_line().unwrap().unwrap(), "H, World");
        remove_file(filename).unwrap();
    }
}
