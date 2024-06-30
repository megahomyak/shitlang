mod input {
    #[derive(Clone, Copy)]
    pub struct Input<'a> {
        s: &'a str,
        index: usize,
    }

    impl<'a> Input<'a> {
        pub fn new(s: &'a str) -> Self {
            Self { s, index: 0 }
        }

        pub fn next(&mut self) -> Option<(usize, char)> {
            unsafe { self.s.get_unchecked(self.index..) }
                .chars()
                .next()
                .map(|c| (self.index, c))
                .inspect(|(_i, c)| self.index += c.len_utf8())
        }
    }
}
use input::Input;

mod utils {
    use super::*;

    pub fn parse_char(mut input: Input, c: char) -> ShitResult<(), ()> {
        match input.next() {
            None => (),
            Some((_i, input_c)) => {
                if input_c == c {
                    return Ok(((), input));
                }
            }
        }
        Err(())
    }

    pub fn parse_word_char(mut input: Input) -> ShitResult<char, ()> {
        if ShitString::parse_delimiter(input.clone()).is_err()
            && Assignment::parse_separator(input.clone()).is_err()
        {
            match input.next() {
                None => (),
                Some((_i, c)) => {
                    if !c.is_whitespace() {
                        return Ok((c, input));
                    }
                }
            }
        }
        Err(())
    }

    pub fn parse_known_word<'a>(mut input: Input<'a>, word: &str) -> ShitResult<'a, (), ()> {
        for word_c in word.chars() {
            match input.next() {
                None => (),
                Some((_i, c)) => {
                    if word_c == c {
                        continue;
                    }
                }
            }
            return Err(());
        }
        match parse_word_char(input.clone()) {
            Err(()) => Ok(((), input)),
            Ok(_) => Err(()),
        }
    }

    #[derive(Clone)]
    pub enum Position {
        ByteOffset(usize),
        EndOfFile,
    }
}
pub use utils::Position;
use utils::*;

macro_rules! shit_mod {
    ($name:ident exports $export:ident { $body:tt }) => {
        pub mod $name {
            use super::*;

            $body
        }
        pub use $name::$export;
    }
}

shit_mod! {function exports Function {
    pub struct Function {
        pub content: Program,
    }

    pub enum Error {}
}}

shit_mod! {if_else exports IfElse {
    pub struct IfElse {
        pub if_branch: Program,
        pub else_branch: Program,
    }

    pub enum Error {}
}}

shit_mod! {shit_loop exports Loop {
    use super::*;

    pub struct Loop {
        pub content: Program,
    }

    pub enum LoopError {}
}}

shit_mod! {name exports Name {
    use super::*;

    pub struct Name {
        pub content: String,
    }

    pub(super) fn parse(input: Input) -> ShitResult {}

    impl Name {
        fn parse(input: Input) -> ShitResult<Self, ()> {}
    }
}}

pub enum NameError {}

// String

pub struct ShitString {
    pub content: String,
}

impl ShitString {
    fn parse_delimiter(input: Input) -> ShitResult<(), ()> {
        utils::parse_char(input, '"')
    }
}

pub enum StringError {}

// Import

pub struct Import {}

pub enum ImportError {}

// Expression

pub enum Expression {
    Function(Function),
    String(ShitString),
    Name(Name),
    IfElse(IfElse),
    Loop(Loop),
    Import(Import),
}

// Assignment

pub struct Assignment {
    pub name: Name,
    pub value: Expression,
}

pub enum AssignmentError {
    MissingName,
    MissingAssignmentSeparator,
    MissingExpression,
}

impl Assignment {
    fn parse_separator(input: Input) -> ShitResult<(), ()> {
        parse_char(input, '=')
    }

    fn parse(input: Input) -> ShitResult<Self, Option<AssignmentError>> {}
}

// Statement

pub enum Statement {
    Assignment(Assignment),
    Expression(Expression),
}

pub struct Program {
    pub statements: Vec<Statement>,
}

pub struct Error<Kind> {
    pub range: std::ops::RangeInclusive<Position>,
    pub kind: Kind,
}

type ShitResult<'a, T, E> = Result<(T, Input<'a>), E>;

fn skip_whitespace(mut input: Input) -> Input {
    loop {
        let input_backup = input.clone();
        match input.next() {
            None => return input,
            Some((_i, c)) => {
                if !c.is_whitespace() {
                    return input_backup;
                }
            }
        }
    }
}

fn one_item_range<T: Clone>(item: T) -> std::ops::RangeInclusive<T> {
    item.clone()..=item
}

impl Program {
    fn parse_end_marker(input: Input) -> ShitResult<(), ()> {
        parse_known_word(input, "end")
    }

    fn parse(input: Input) -> ShitResult<Program, Error<AssignmentError>> {
        let mut assignments = Vec::new();
    }
}

pub fn parse(input: &str) -> Result<Program, Error<AssignmentError>> {
    Program::parse(Input::new(input)).map(|(program, _rest)| program)
}
