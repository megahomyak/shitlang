#[derive(Clone, Copy)]
pub struct Pos {
    col: usize,
    row: usize,
}

const FIRST_COL: usize = 0;
const FIRST_ROW: usize = 1;

impl Pos {
    pub fn new() -> Self {
        Self { col: FIRST_COL, row: FIRST_ROW }
    }

    pub fn update(&mut self, c: char) {
        match c {
            '\n' => {
                self.col = FIRST_COL;
                self.row += 1;
            },
            _ => {
                self.col += 1;
            }
        }
    }
}
