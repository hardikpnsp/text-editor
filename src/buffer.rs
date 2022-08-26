use std::fs::File;
use std::io;
use std::io::BufRead;

pub struct Buffer {
    rows: Vec<String>,
    row: u32,
    col: u32,
}

impl Buffer {
    pub fn new(filename: &str) -> Self {
        let file = File::open(filename).expect("could not open file");
        let mut buffer: Vec<String> = vec![];

        for line in io::BufReader::new(file).lines() {
            let line = line.expect("failed reading line");
            buffer.push(line);
        }

        let current_row: u32 = (buffer.len() - 1) as u32;

        return Buffer {
            rows: buffer,
            row: current_row,
            col: 0,
        };
    }

    pub fn rows(&self) -> &Vec<String> {
        return &self.rows;
    }

    pub fn write(&mut self, char: char) {
        match char {
            '\n' => {
                self.rows.push(String::new());
                self.row += 1;
            },
            _ => {
                self.rows[self.row as usize].push(char);
            }
        }
    }
}
