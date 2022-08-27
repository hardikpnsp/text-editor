use std::fs::File;
use std::io;
use std::io::{BufRead, LineWriter, Write};

use crate::cursor::Cursor;

pub struct Buffer {
    rows: Vec<String>,
    pub cursor: Cursor,
}

impl Buffer {
    pub fn new(filename: &str) -> Self {
        let file = File::open(filename).expect("could not open file");
        let mut buffer: Vec<String> = vec![];

        for line in io::BufReader::new(file).lines() {
            let line = line.expect("failed reading line");
            buffer.push(line);
        }

        return Buffer {
            rows: buffer,
            cursor: Cursor { row: 0, col: 0 }
        };
    }

    pub fn rows(&self) -> &Vec<String> {
        return &self.rows;
    }

    pub fn write(&mut self, char: char) {
        self.adjust_cursor_boundary_before_edit();

        match char {
            '\n' => {
                let line = &self.rows[self.cursor.row];
                if self.cursor.col >= line.len() {
                    self.rows.insert( self.cursor.row + 1, String::new());
                    self.cursor.new_line();
                } else {
                    let cur = &line[..self.cursor.col];
                    let next = &line[self.cursor.col..];
                    let result = next.to_string();
                    self.rows[self.cursor.row] = cur.to_string();
                    self.rows.insert(self.cursor.row + 1, result);
                    self.cursor.new_line();
                }
            },
            _ => {
                let line = &self.rows[self.cursor.row];
                if self.cursor.col > line.len() {
                    self.rows[self.cursor.row].push(char);
                } else {
                    let pre = &line[..self.cursor.col];
                    let post = &line[self.cursor.col..];
                    let mut result = pre.to_string();
                    result.push(char);
                    result.push_str(post);
                    self.rows[self.cursor.row] = result;
                    self.cursor.right();
                }
            }
        }
    }

    fn adjust_cursor_boundary_before_edit(&mut self) {
        if self.cursor.row > self.rows.len() {
            self.cursor.goto(self.rows.len() - 1, self.rows[self.rows.len() - 1].len());
        }
        if self.cursor.col > self.rows[self.cursor.row].len() {
            self.cursor.goto(self.cursor.row, self.rows[self.cursor.row].len());
        }
    }

    pub fn delete(&mut self) {
        self.adjust_cursor_boundary_before_edit();

        if self.cursor.col == 0 {
            if self.cursor.row != 0 {
                let l = self.rows[self.cursor.row - 1].len();

                let current_line = self.rows[self.cursor.row].clone();
                self.rows[self.cursor.row - 1].push_str(&*current_line);

                self.cursor.delete_line(l);
                self.rows.remove(self.cursor.row + 1);
            }
        } else {
            let line = &self.rows[self.cursor.row];
            let cur = &line[..self.cursor.col];
            let next = &line[self.cursor.col..];
            let mut result = cur.to_string();
            result.pop();
            result.push_str(next);
            self.rows[self.cursor.row] = result;
            self.cursor.left();
        }
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename).expect("could not open file in write only mode");
        let mut file = LineWriter::new(file);

        for row in &self.rows {
            file.write_all(row.as_ref())?;
            file.write_all(b"\r\n")?;
        }

        file.flush()?;
        Ok(())
    }
}
#[cfg(test)]
mod test {
    use std::fs::{File, remove_file};
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