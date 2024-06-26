pub type BoolShit<'a> = Option<Rest<'a>>;
pub type SureShit<'a, T> = Option<(T, Rest<'a>)>;

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

const FIRST_COL: usize = 0;

impl<'a> From<&'a str> for Rest<'a> {
    fn from(contents: &'a str) -> Self {
        Self {
            contents,
            pos: Position {
                col: FIRST_COL,
                row: 1,
            },
        }
    }
}

impl<'a> Rest<'a> {
    pub fn trim(&self) -> Option<(char, Self)> {
        let mut chars = self.contents.chars();
        chars.next().map(|c| {
            (
                c,
                Self {
                    contents: chars.as_str(),
                    pos: match c {
                        '\n' => Position {
                            col: FIRST_COL,
                            row: self.pos.row + 1,
                        },
                        _ => Position {
                            col: self.pos.col + 1,
                            row: self.pos.row,
                        },
                    },
                },
            )
        })
    }
}

pub fn skip_whitespace(mut rest: Rest) -> Rest {
    while let Some((c, new_rest)) = rest.trim() {
        if !c.is_whitespace() {
            break;
        }
        rest = new_rest;
    }
    rest
}

pub fn char(rest: Rest, filter: impl FnOnce(char) -> bool) -> SureShit<char> {
    rest.trim()
        .and_then(|(c, rest)| filter(c).then(|| (c, rest)))
}
