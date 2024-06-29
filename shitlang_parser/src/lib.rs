pub enum Token<'a> {
    Whitespace(&'a str),
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

struct Pos {
    col: usize,
    row: usize,
}

enum Error {
    QuoteNotClosed { pos: Pos },
}

struct Tokenizer<'a> {
    program_parts: Vec<ProgramPart<'a>>,
}

impl<'a> Tokenizer<'a> {
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
    Tokenizer {
        program_parts: Vec::new(ProgramPart::Raw(program)),
    }
    .concretize(|s| s.split('"'))
}

#[cfg(test)]
mod tests {}
