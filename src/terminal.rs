pub struct Terminal;

impl Terminal {
    pub fn rows() -> usize {
        termion::terminal_size().unwrap().1 as usize
    }

    pub fn cols() -> usize {
        termion::terminal_size().unwrap().0 as usize
    }
}
