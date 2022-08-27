use termion::cursor::Goto;

pub struct Cursor {
    row: usize,
    col: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {row: 0, col: 0}
    }
    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn right(&mut self) {
        self.goto(self.row, self.col + 1);
    }

    pub fn left(&mut self) {
        if self.col > 0 {
            self.goto(self.row, self.col - 1);
        }
    }

    pub fn new_line(&mut self) {
        self.goto(self.row + 1, 0);
    }

    pub fn up(&mut self) {
        if self.row > 0 {
            self.goto(self.row - 1, self.col);
        }
    }

    pub fn down(&mut self) {
        self.goto(self.row + 1, self.col);
    }

    pub fn delete_line(&mut self, previous_line_len: usize) {
        if self.row > 0 {
            self.goto(self.row - 1, previous_line_len);
        }
    }

    pub fn goto(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
        print!("{}", Goto(self.col as u16 + 1, self.row as u16 + 1));
    }
}
