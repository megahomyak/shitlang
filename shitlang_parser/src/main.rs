type BoolShit<'a> = Option<Rest<'a>>;
type SureShit<'a, T> = Option<(T, Rest<'a>)>;
type ShitResult<'a, T> = Result<SureShit<'a, T>, ParsingError>;

#[derive(Clone, Copy)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

#[derive(Clone, Copy)]
struct Rest<'a> {
    contents: &'a str,
    pos: Position,
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
    fn trim(&self) -> Option<(char, Self)> {
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

pub enum ParsingError {}

pub enum Value {
    VariableName(String),
    Import(String),
    String(String),
    If(Program, Program),
    Loop(Program),
    Command(Program),
}

pub struct Assignment {
    pub name: String,
    pub value: Value,
}

pub struct Program {
    pub assignments: Vec<Assignment>,
}

fn skip_whitespace(mut rest: Rest) -> Rest {
    while let Some((c, new_rest)) = rest.trim() {
        if !c.is_whitespace() {
            break;
        }
        rest = new_rest;
    }
    rest
}

fn char(rest: Rest, filter: impl FnOnce(char) -> bool) -> SureShit<char> {
    rest.trim()
        .and_then(|(c, rest)| filter(c).then(|| (c, rest)))
}

const IMPORT: &'static str = "import";
const IF: &'static str = "if";
const ELSE: &'static str = "else";
const END: &'static str = "end";
const LOOP: &'static str = "loop";
const COMMAND: &'static str = "command";
const KEYWORDS: [&'static str; 6] = [IMPORT, IF, ELSE, END, LOOP, COMMAND];
const ASSIGNMENT_SYMBOL: char = '=';
const STRING_BOUNDARY: char = '"';

fn keyword<'a>(mut rest: Rest<'a>, keyword: &str) -> BoolShit<'a> {
    let mut keyword = keyword.chars();
    while let Some((c, new_rest)) = rest.trim() {
        match keyword.next() {
            None => return Some(rest),
            Some(kw_c) => {
                if kw_c != c {
                    return None;
                }
            }
        }
        rest = new_rest;
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn keyword() {
        assert_eq!(keyword())
    }
}

fn varname(mut rest: Rest) -> SureShit<String> {
    let mut name = String::new();
    while let Some((c, new_rest)) = char(rest, |c| {
        !(c.is_whitespace() || [ASSIGNMENT_SYMBOL, STRING_BOUNDARY].contains(&c))
    }) {
        name.push(c);
        rest = new_rest;
    }
    if name.is_empty() || KEYWORDS.contains(&&name[..]) {
        None
    } else {
        name.shrink_to_fit();
        Some((name, rest))
    }
}

pub fn parse(program: &str) -> Result<Program, ParsingError> {
    let rest: Rest = program.into();
}
