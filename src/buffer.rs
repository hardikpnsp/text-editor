use std::fs::File;
use std::io;
use std::io::{BufRead, LineWriter, Write};
use termion::cursor::{Left, Right};

pub struct Cursor {
    row: usize,
    col: usize,
}

impl Cursor {
    pub fn right(&mut self) {
        print!("{}", Right(1));
        self.col += 1;
    }

    pub fn left(&mut self) {
        print!("{}", Left(1));
        self.col -= 1;
    }

    fn new_line(&mut self) {
        self.row += 1;
        self.col = 0;
        print!("{}", termion::cursor::Goto(1, self.row as u16 + 1));
    }

    pub fn up(&mut self) {
        self.row -= 1;
        print!("{}", termion::cursor::Goto(self.col as u16 + 1, self.row as u16 + 1));
    }

    pub fn down(&mut self) {
        self.row += 1;
        print!("{}", termion::cursor::Goto(self.col as u16 + 1, self.row as u16 + 1));
    }

    fn delete_line(&mut self, previous_line_len: usize) {
        if self.row > 0 {
            self.row -= 1;
            self.col = previous_line_len;
            print!("{}", termion::cursor::Goto(self.col as u16 + 1, self.row as u16 + 1));
        }
    }
}

pub struct Buffer {
    rows: Vec<String>,
    pub cursor: Cursor
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
                if self.cursor.col >= line.len() {
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

    pub fn delete(&mut self) {
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

    use crate::buffer::Buffer;

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