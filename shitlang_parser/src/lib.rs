mod input {
    #[derive(Clone)]
    pub struct Input<'a> {
        iter_output: Option<(usize, char)>,
        iter: std::str::CharIndices<'a>,
    }

    impl<'a> Input<'a> {
        pub fn new(s: &'a str) -> Self {
            let mut iter = s.char_indices();
            Self {
                iter_output: iter.next(),
                iter,
            }
        }

        pub fn peek(&self) -> Option<(usize, char)> {
            self.iter_output
        }

        pub fn advance(&mut self) {
            self.iter_output = self.iter.next();
        }
    }
}
use input::Input;

#[derive(Clone)]
pub enum Position {
    ByteOffset(usize),
    EndOfFile,
}

pub struct ByteOffset(usize);
pub struct Inclusive<T>(T);
pub struct Exclusive<T>(T);

pub enum Range {
    EndOfInput,
    ToEndOfInput { beginning: Inclusive<ByteOffset> },

}

pub struct Function {
    pub content: Program,
}

pub enum FunctionError {}

pub struct IfElse {
    pub if_branch: Program,
    pub else_branch: Program,
}

pub enum IfElseError {}

pub struct Loop {
    pub content: Program,
}

pub enum LoopError {}

pub struct Name {
    pub content: String,
}

impl Name {
    pub fn parse(mut input: Input) -> ShitResult<Self, ()> {
        let mut content = String::new();
        while let Some((_i, c)) = input.peek() {
            match c {
                '"' | '=' => break,
                c if c.is_whitespace() => break,
                c => content.push(c),
            }
            input.advance();
        }
        if content.is_empty() {
            Err(())
        } else {
            content.shrink_to_fit();
            Ok((Self { content }, input))
        }
    }
}

pub enum NameError {}

pub struct ShitString {
    pub content: String,
}

pub enum StringError {}

pub struct Import {}

pub enum ImportError {}

pub enum Expression {
    Function(Function),
    String(ShitString),
    Name(Name),
    IfElse(IfElse),
    Loop(Loop),
    Import(Import),
}

pub struct Assignment {
    pub name: Name,
    pub value: Expression,
}

pub enum AssignmentError {
    MissingName,
    MissingEqualsSign,
    MissingExpression,
}

impl Assignment {
    fn parse(input: Input) -> ShitResult<Self, AssignmentError> {
        let input = skip_whitespace(input);
        let Ok((var_name, input)) = Name::parse(input.clone()) else {
            return Err(Error {
                range: make_range(match input.peek() {
                    Some((i, _c)) => Position::ByteOffset(i),
                    None => Position::EndOfFile,
                }),
                kind: AssignmentError::MissingName,
            });
        };
        let input = skip_whitespace(input);
        match input.peek() {
            None => Err(Error {
                range: make_range(Position::EndOfFile),
                kind: ErrorKind::
            }),
        }
    }
}

pub struct Program {
    pub assignments: Vec<Assignment>,
}

pub enum ErrorKind {
    AssignmentError(AssignmentError),
}

pub struct Error {
    pub range: std::ops::RangeInclusive<Position>,
    pub kind: ErrorKind,
}

pub type ShitResult<'a, T, E> = Result<(T, Input<'a>), E>;

fn skip_whitespace(mut input: Input) -> Input {
    while let Some((_i, c)) = input.peek() {
        if !c.is_whitespace() {
            break;
        }
        input.advance();
    }
    input
}

fn make_range<T: Clone>(item: T) -> std::ops::RangeInclusive<T> {
    item.clone()..=item
}

impl Program {
}

pub fn parse(input: &str) -> Result<Program, Error> {
    Program::parse(Input::new(input)).map(|(program, _rest)| program)
}
