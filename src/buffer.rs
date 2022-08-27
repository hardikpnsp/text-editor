use crate::cursor::Cursor;
use std::fs::File;
use std::io;
use std::io::{BufRead, LineWriter, Write};
use termion::terminal_size;

struct Line {
    value: String,
    display_rows: usize,
}

impl Line {
    pub fn from(value: String) -> Self {
        Line {
            value,
            display_rows: 1,
        }
    }

    pub fn render(&mut self) {
        let (col, _row) = terminal_size().unwrap();
        if col < self.value.len() as u16 {
            let mut display_rows = 0;
            let mut cur = 0;
            let step: usize = (col).into();
            while cur + step < self.value.len() {
                print!("{}\r\n", self.value[cur..(cur + step)].to_string());
                cur += step;
                display_rows += 1;
            }
            print!("{}\r\n", self.value[cur..].to_string());
            display_rows += 1;

            self.display_rows = display_rows;
        } else {
            print!("{}\r\n", self.value);
            self.display_rows = 1;
        }
    }
}

pub struct Buffer {
    lines: Vec<Line>,
    pub cursor: Cursor,
}

impl Buffer {
    pub fn buffer_row(&self) -> usize {
        let cursor_row = self.cursor.row();
        let mut buffer_row = 0;

        let mut cur = 0;

        while cur < cursor_row {
            cur += self.lines[buffer_row].display_rows;
            if cur <= cursor_row {
                buffer_row += 1;
            }
        }

        buffer_row
    }

    pub fn buffer_row_start(&self, buffer_row :usize) ->usize {
        let mut cursor_row = 0;

        for line in 0..buffer_row {
            cursor_row += self.lines[line].display_rows;
        }

        cursor_row
    }

    pub fn buffer_col(&self) -> usize {
        let cursor_row = self.cursor.row();
        let cursor_col = self.cursor.col();

        let buffer_row = self.buffer_row();
        let cursor_row_offset = cursor_row - self.buffer_row_start(buffer_row);

        let (col, _row) = terminal_size().unwrap();

        ((col as usize) * cursor_row_offset) + cursor_col
    }

    pub fn last_cursor_row(&self) -> usize {
        let mut cursor_row = 0;
        for line in &self.lines {
            cursor_row += line.display_rows;
        }

        cursor_row
    }

    pub fn last_cursor_col(&self, buffer_row: usize) -> usize {
        let line_length = self.lines[buffer_row].value.len();

        let (col, _row) = terminal_size().unwrap();

        let remainder = line_length % col as usize;

        return remainder
    }

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
        let row = self.buffer_row();
        let col = self.buffer_col();
        match char {
            '\n' => {
                let line = &self.lines[row].value;
                if col >= line.len() {

                    self.lines
                        .insert(row + 1, Line::from(String::new()));
                    self.cursor.new_line();
                } else {
                    let cur = &line[..col];
                    let next = &line[col..];
                    let result = next.to_string();
                    self.lines[row].value = cur.to_string();
                    self.lines.insert(row + 1, Line::from(result));
                    self.cursor.new_line();
                }
            }
            _ => {
                let line = &self.lines[row].value;
                if col > line.len() {
                    self.lines[row].value.push(char);
                } else {
                    let pre = &line[..col];
                    let post = &line[col..];
                    let mut result = pre.to_string();
                    result.push(char);
                    result.push_str(post);
                    self.lines[row].value = result;
                    self.cursor.right();
                }
            }
        }
    }

    fn adjust_cursor_boundary_before_edit(&mut self) {
        let last_cursor_row = self.last_cursor_row();
        if self.cursor.row() > last_cursor_row {
            let last_cursor_column = self.last_cursor_col(self.lines.len() - 1);
            self.cursor.goto(
                last_cursor_row,
                last_cursor_column,
            );
        }
        let row = self.buffer_row();
        let col = self.buffer_col();

        if col >= self.lines[row].value.len() {
            let last_cursor_column = self.last_cursor_col(row);
            self.cursor
                .goto(self.cursor.row(), last_cursor_column);
        }
    }

    pub fn delete(&mut self) {
        self.adjust_cursor_boundary_before_edit();
        let row = self.buffer_row();
        let col = self.buffer_col();
        if col == 0 {
            if row != 0 {
                let l = self.lines[row - 1].value.len();

                let current_line = self.lines[row].value.clone();
                self.lines[row - 1]
                    .value
                    .push_str(&*current_line);

                self.cursor.delete_line(l);
                self.lines.remove(row);
            }
        } else {
            let line = &self.lines[row].value;
            let cur = &line[..col];
            let next = &line[col..];
            let mut result = cur.to_string();
            result.pop();
            result.push_str(next);
            self.lines[row].value = result;
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

    pub fn render(&mut self) {
        for line in &mut self.lines {
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
