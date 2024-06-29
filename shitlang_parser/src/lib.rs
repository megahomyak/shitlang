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

pub struct Inclusive<T>(T);
pub struct Exclusive<T>(T);

pub struct Range {
    beginning: Inclusive<Position>,
    end: Exclusive<Position>,
}

pub mod function {
    pub enum Error {

    }

    pub struct Function {
        pub content: Program,
    }
}
pub use function::Function;

pub mod if_else {
    pub enum Error {}

    pub struct IfElse {
        pub if_branch: Program,
        pub else_branch: Program,
    }
}
pub use if_else::IfElse;

pub mod shit_loop {
    pub enum Error {}

    pub struct Loop {
        pub content: Program,
    }
}

pub mod variable_name {
    pub enum Error {}

    pub struct VariableName {
        pub content: String,
    }

impl VariableName {
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

}

mod string {
    pub enum Error {}

    pub struct ShitString {
        pub content: String,
    }
}

mod expression {
pub enum Expression {
    Function(Function),
    String(ShitString),
    VariableName(VariableName),
    IfElse(IfElse),
    Loop(Loop),
}
}

mod assignment {
    pub enum Error {}

pub struct Assignment {
    pub name: VariableName,
    pub value: Expression,
}
}

pub struct Program {
    pub assignments: Vec<Assignment>,
}

pub enum ErrorKind {
    AssignmentError(AssignmentError)
    MissingNameInAssignment,
    MissingEqualsSignInAssignment,
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
    fn parse(input: Input) -> ShitResult<Self, Error> {
        let input = skip_whitespace(input);
        let Ok((var_name, input)) = VariableName::parse(input.clone()) else {
            return Err(Error {
                range: make_range(match input.peek() {
                    Some((i, _c)) => Position::ByteOffset(i),
                    None => Position::EndOfFile,
                }),
                kind: ErrorKind::MissingNameInAssignment,
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

pub fn parse(input: &str) -> Result<Program, Error> {
    Program::parse(Input::new(input)).map(|(program, _rest)| program)
}
