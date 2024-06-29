enum Expression {
    Function(Program),
    String(String),
    VarName(String),
    IfElse(Program, Program),
    Loop(Program),
}

struct Assignment {
    pub name: String,
    pub value: Expression,
}

pub struct Program {
    pub assignments: Vec<Assignment>,
}

pub enum Error {
    NoNameInAssignment {
        index: usize,
    }
}

pub fn parse(s: &str) -> Result<Program, Error> {
    let mut chars = s.char_indices().peekable();
    let mut assignments = Vec::new();
    if let Some((i, c)) = chars.peek() {
        if *c == '=' {
            return Err(Error::NoNameInAssignment { index: *i });
        } else if c.is_whitespace() {}
        else {
        
        }
    } else {
        return Ok(Program { assignments });
    }
}
