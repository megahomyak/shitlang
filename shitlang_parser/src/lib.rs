pub enum Token<'a> {
    WhiteSpace(&'a str),
    VarName(&'a str),
    Function,
    Assignment,
    If,
    Else,
    Loop,
    End,
}

pub enum ProgramPart<'a> {
    Token(Token<'a>),
    Raw(&'a str),
}

#[derive(Clone)]
struct Pos {
    col: usize,
    row: usize,
}

const FIRST_COL: usize = 0;
const FIRST_ROW: usize = 1;

impl Pos {
    pub fn new() -> Self {
        Self { col: FIRST_COL, row: FIRST_ROW }
    }

    pub fn update(&mut self, s: &str) {
        for c in s.chars() {
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
}

enum Error {
    QuoteNotClosed { pos: Pos },
}

struct Program<'a> {
    cur_pos: Pos,
    program_parts: Vec<ProgramPart<'a>>,
}

impl<'a> Program<'a> {
    fn concretize(
        self,
        f: impl Fn(&'a str) -> Result<Vec<ProgramPart<'a>>, Error>,
    ) -> Result<Self, Error> {
        let mut new_program_parts = Vec::new();
        for program_part in self.program_parts {
            match program_part {
                ProgramPart::Token(_) => new_program_parts.push(program_part),
                ProgramPart::Raw(s) => new_program_parts.extend(f(s)?),
            }
        }
        Ok(Self {
            program_parts: new_program_parts,
        })
    }

    fn finalize(self, f: impl Fn(&'a str) -> Vec<Token<'a>>) -> Result<Vec<Token<'a>>, Error> {
        match self {
            Self::Fucked { error } => Err(error),
            Self::Right { program_parts } => Ok({
                let mut tokens = Vec::new();
                for program_part in self.program_parts {
                    match program_part {
                        ProgramPart::Token(token) => tokens.push(token),
                        ProgramPart::Raw(s) => tokens.extend(f(s)),
                    }
                }
                tokens
            }),
        }
    }
}

fn tokenize<'a>(program: &'a str) -> Vec<Token<'a>> {
    Program {
        program_parts: Vec::new(ProgramPart::Raw(program)),
    }
    .concretize(|s| s.split('"'))
}

#[cfg(test)]
mod tests {}
