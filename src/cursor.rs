use termion::cursor::{Left, Right};

pub struct Cursor {
    pub row: usize,
    pub col: usize,
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

    pub fn new_line(&mut self) {
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

    pub fn delete_line(&mut self, previous_line_len: usize) {
        if self.row > 0 {
            self.row -= 1;
            self.col = previous_line_len;
            print!("{}", termion::cursor::Goto(self.col as u16 + 1, self.row as u16 + 1));
        }
    }
}
