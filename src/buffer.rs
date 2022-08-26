use std::fs::File;
use std::io;
use std::io::{BufRead, LineWriter, Write};

struct Cursor {
    row: usize,
    col: usize,
}

pub struct Buffer {
    rows: Vec<String>,
    cursor: Cursor
}

impl Buffer {
    pub fn new(filename: &str) -> Self {
        let file = File::open(filename).expect("could not open file");
        let mut buffer: Vec<String> = vec![];

        for line in io::BufReader::new(file).lines() {
            let line = line.expect("failed reading line");
            buffer.push(line);
        }

        let current_row: usize = buffer.len() - 1;

        return Buffer {
            rows: buffer,
            cursor: Cursor { row: current_row, col: 0 }
        };
    }

    pub fn rows(&self) -> &Vec<String> {
        return &self.rows;
    }

    pub fn write(&mut self, char: char) {
        match char {
            '\n' => {
                self.rows.push(String::new());
                self.cursor.row += 1;
            },
            _ => {
                self.rows[self.cursor.row].push(char);
            }
        }
    }

    pub fn delete(&mut self) {
        if self.rows[self.cursor.row].len() > 0 {
            self.rows[self.cursor.row].pop();
        } else if self.rows.len() > 0 {
            self.rows.pop();
            self.cursor.row -= 1;
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
    use std::io::Write;

    use termion::input::TermRead;

    use crate::buffer::Buffer;

    #[test]
    fn buffer_writes_saves_and_deletes() {
        let filename = "write_test_file.txt";
        let mut f = File::create(filename).unwrap();
        f.write_all(b"Hello, ").unwrap();

        let mut buffer = Buffer::new(filename);
        buffer.write('W');
        buffer.write('o');
        buffer.write('r');
        buffer.write('l');
        buffer.write('d');

        buffer.save(filename).unwrap();

        let mut file = File::open(filename).unwrap();
        assert_eq!(file.read_line().unwrap().unwrap(), "Hello, World");

        buffer.delete();
        buffer.delete();
        buffer.delete();
        buffer.delete();
        buffer.delete();

        buffer.save(filename).unwrap();
        let mut file = File::open(filename).unwrap();
        assert_eq!(file.read_line().unwrap().unwrap(), "Hello, ");
        remove_file(filename).unwrap();
    }
}