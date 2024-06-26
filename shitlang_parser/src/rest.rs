#[derive(Clone, Copy)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

#[derive(Clone, Copy)]
pub struct Rest<'a> {
    pub contents: &'a str,
    pub pos: Position,
}
