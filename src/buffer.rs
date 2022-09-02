use crate::cursor::Cursor;
use std::fs::File;
use std::io;
use std::io::{BufRead, LineWriter, Write};
use termion::terminal_size;

struct Line {
    value: String,
}

impl Line {
    pub fn from(value: String) -> Self {
        Line { value }
    }

    pub fn display_rows(&self, is_wrap: bool) -> usize {
        if !is_wrap {
            return 1;
        }

        if self.value.len() == 0 {
            return 1;
        } else {
            let (col, _row) = terminal_size().unwrap();
            let mut div = self.value.len() / col as usize;
            if self.value.len() % col as usize != 0 {
                div += 1;
            }
            return div;
        }
    }

    pub fn render(&mut self) {
        // renders the content of `self.value`, can take multiple terminal rows due to wrapping
        let (col, _row) = terminal_size().unwrap();
        for i in 0..self.display_rows(true) {
            print!(
                "{}\r\n",
                self.value[(i * col as usize)..(((i + 1) * col as usize).min(self.value.len()))]
                    .to_string()
            );
        }
    }

    pub fn render_no_wrap(&mut self, cursor_col_pos: usize) {
        let (col, _row) = terminal_size().unwrap();
        if cursor_col_pos > col as usize {
            let left_offset = (cursor_col_pos - col as usize).min(self.value.len());
            let right_offset = (left_offset + col as usize).min(self.value.len());
            print!("{}\r\n", self.value[left_offset..right_offset].to_string());
        } else {
            print!(
                "{}\r\n",
                self.value[..(col as usize).min(self.value.len())].to_string()
            );
        }
    }
}

pub struct Buffer {
    // stores the entire file as a vector of `Line`s
    // provides facility for editing and saving the file content
    lines: Vec<Line>,
    pub cursor: Cursor,
    pub top_offset: usize,
    filename: String,
    is_wrap: bool,
}

impl Buffer {
    pub fn new(filename: &str) -> Result<Self, ()> {
        if let Ok(file) = File::open(filename) {
            let mut buffer: Vec<Line> = vec![];

            for line in io::BufReader::new(file).lines() {
                let line = line.expect("failed reading line");
                buffer.push(Line::from(line));
            }

            Ok(Buffer {
                lines: buffer,
                cursor: Cursor::new(),
                top_offset: 0,
                filename: filename.to_string(),
                is_wrap: false,
            })
        } else {
            Err(())
        }
    }

    pub fn buffer_row(&self) -> usize {
        // Maps the current cursor row in terminal window with the row in `self.lines`
        let cursor_row = self.cursor.row();
        let mut buffer_row = self.top_offset;

        let mut total_cursor_rows = 0;

        for i in self.top_offset..self.lines.len() {
            total_cursor_rows += self.lines[i].display_rows(self.is_wrap);
            if total_cursor_rows > cursor_row {
                buffer_row = i;
                break;
            }
        }

        buffer_row
    }

    pub fn buffer_row_start(&self, buffer_row: usize) -> usize {
        // Finds the starting cursor row in terminal for the given row in buffer
        self.lines[self.top_offset..buffer_row]
            .iter()
            .map(|line| line.display_rows(self.is_wrap))
            .sum()
    }

    pub fn buffer_col(&self) -> usize {
        // Maps the current cursor column in terminal window with the column in `self.lines[self.buffer_row()]`

        let cursor_row = self.cursor.row();
        let cursor_col = self.cursor.col();

        let buffer_row = self.buffer_row();
        let cursor_row_offset = cursor_row - self.buffer_row_start(buffer_row);

        let (col, _row) = terminal_size().unwrap();

        ((col as usize) * cursor_row_offset) + cursor_col
    }

    pub fn last_cursor_row(&self) -> usize {
        // Calculates the last row on terminal for all the lines
        self.lines[self.top_offset..]
            .iter()
            .map(|line| line.display_rows(self.is_wrap))
            .sum()
    }

    pub fn last_cursor_col(&self, buffer_row: usize) -> usize {
        // Calculates the last column for the cursor for given row
        let line_length = self.lines[buffer_row].value.len();

        if !self.is_wrap {
            return line_length;
        }

        let (col, _row) = terminal_size().unwrap();

        line_length % col as usize
    }

    pub fn write(&mut self, char: char) {
        self.adjust_cursor_boundary_before_edit();
        let row = self.buffer_row();
        let col = self.buffer_col();
        match char {
            '\n' => {
                let line = &self.lines[row].value;
                if col >= line.len() {
                    self.lines.insert(row + 1, Line::from(String::new()));
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
        // If cursor is not on text, bring it back to text to avoid out of bounds
        let last_cursor_row = self.last_cursor_row();
        if self.cursor.row() > last_cursor_row {
            let last_cursor_column = self.last_cursor_col(self.lines.len() - 1);
            self.cursor.goto(last_cursor_row, last_cursor_column);
        }
        let row = self.buffer_row();
        let col = self.buffer_col();

        if col >= self.lines[row].value.len() {
            let last_cursor_column = self.last_cursor_col(row);
            self.cursor.goto(self.cursor.row(), last_cursor_column);
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
                self.lines[row - 1].value.push_str(&*current_line);

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

    pub fn down(&mut self) {
        if self.cursor.row() >= self.last_cursor_row() - 1 {
            return;
        }
        if self.cursor.row() + 2 >= termion::terminal_size().unwrap().1 as usize {
            self.top_offset += 1;
        } else {
            self.cursor.down();
        }
    }

    pub fn up(&mut self) {
        if self.cursor.row() == 0 && self.top_offset > 0 {
            self.top_offset -= 1;
        } else if self.cursor.row() > 0 {
            self.cursor.up();
        }
    }

    pub fn left(&mut self) {
        self.cursor.left();
    }

    pub fn right(&mut self) {
        self.cursor.right();
    }

    pub fn save(&self) -> std::io::Result<()> {
        let file = File::create(&self.filename).expect("could not open file in write only mode");
        let mut file = LineWriter::new(file);

        for line in &self.lines {
            file.write_all(line.value.as_ref())?;
            file.write_all(b"\r\n")?;
        }

        file.flush()?;
        Ok(())
    }

    pub fn toggle_wrapping(&mut self) {
        self.is_wrap = !self.is_wrap;
    }

    pub fn render(&mut self) {
        let (_col, row) = termion::terminal_size().unwrap();
        let mut rows_to_draw: usize = self.top_offset + (row as usize - 1);

        if self.is_wrap {
            let mut cur = self.top_offset;
            let mut console_rows = 0 as usize;
            for i in self.top_offset..self.lines.len() {
                console_rows += self.lines[i].display_rows(self.is_wrap);
                if console_rows <= (row - 1) as usize {
                    cur = i;
                } else {
                    break;
                }
            }
            rows_to_draw = cur + 1;
        }

        if rows_to_draw > self.lines.len() {
            rows_to_draw = self.lines.len();
        }

        for line in &mut self.lines[self.top_offset..rows_to_draw] {
            if self.is_wrap {
                line.render();
            } else {
                line.render_no_wrap(self.cursor.col() + 1);
            }
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

        buffer.save().unwrap();

        let mut file = File::open(filename).unwrap();
        assert_eq!(file.read_line().unwrap().unwrap(), "Hello, World");

        buffer.write('\n');
        buffer.save().unwrap();

        let mut file = File::open(filename).unwrap();
        let mut result = String::new();
        file.read_to_string(&mut result).unwrap();
        assert_eq!(result, "Hello\r\n, World\r\n");

        buffer.delete();
        buffer.delete();
        buffer.delete();
        buffer.delete();
        buffer.delete();

        buffer.save().unwrap();
        let mut file = File::open(filename).unwrap();
        assert_eq!(file.read_line().unwrap().unwrap(), "H, World");
        remove_file(filename).unwrap();
    }
}
