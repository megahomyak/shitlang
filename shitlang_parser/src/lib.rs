type Input<'a> = std::iter::Peekable<std::str::CharIndices<'a>>;

#[derive(Clone, Copy)]
struct Input<'a> {
    s: &'a str,
    index: usize,
    c: Option<char>,
}

impl<'a> Input<'a> {
    fn advance(&mut self) {
        if let Some(c) = self.c {
            self.index += c.len_utf8();
            self.c = self.s.get(self.index..).map;
        }
    }
}

pub struct Function {
    pub content: Program,
}

pub struct IfElse {
    pub if_branch: Program,
    pub else_branch: Program,
}

pub struct Loop {
    pub content: Program,
}

pub struct VarName {
    pub content: String,
}

impl VarName {
    pub fn parse(input: Input) -> Result<Self> {}
}

pub struct ShitString {
    pub content: String,
}

pub enum Expression {
    Function(Function),
    String(ShitString),
    VarName(VarName),
    IfElse(IfElse),
    Loop(Loop),
}

pub struct Assignment {
    pub name: VarName,
    pub value: Expression,
}

pub struct Program {
    pub assignments: Vec<Assignment>,
}

pub enum Error {
    NoNameInAssignment { index: Index },
}

pub type Result<'a, T, E = Option<Error>> = std::result::Result<(T, Input<'a>), E>;

fn skip_whitespace(mut input: Input) -> Input {
    while let Some(c) = input.c {
        if !c.is_whitespace() {
            break;
        }
        input.advance();
    }
    input
}

impl Program {
    fn parse(input: Input) -> Result<Self, Error> {
        let input = skip_whitespace(input);
        let Ok((var_name, input)) = VarName::parse(input) else {
            return Err(Error::NoNameInAssignment {
                index: input.index,
            });
        };
        let input = skip_whitespace(input);
        let mut chars = input.chars();
        match chars.next() {
            '=' => (),
            _ => return,
        }
    }
}

pub fn parse(input: &str) -> std::result::Result<Program, Error> {
    Program::parse(input).map(|(program, _rest)| program)
}
