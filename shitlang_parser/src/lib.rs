mod input {
    #[derive(Clone, Copy)]
    pub(super) struct Input<'a> {
        s: &'a str,
        index: usize,
    }

    impl<'a> Input<'a> {
        pub(super) fn new(s: &'a str) -> Self {
            Self { s, index: 0 }
        }

        pub(super) fn next(&mut self) -> Option<(usize, char)> {
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

    pub(super) fn parse_char(mut input: Input, c: char) -> ShitResult<(), ()> {
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

    pub(super) fn parse_word_char(mut input: Input) -> ShitResult<char, ()> {
        if string::parse_beginning_marker(input).is_err()
            && assignment::parse_separator(input).is_err()
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

    pub(super) fn parse_known_word<'a>(mut input: Input<'a>, word: &str) -> ShitResult<'a, (), ()> {
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
        match parse_word_char(input) {
            Err(()) => Ok(((), input)),
            Ok(_) => Err(()),
        }
    }

    pub(super) fn skip_whitespace(mut input: Input) -> Input {
        loop {
            let input_backup = input;
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

    pub(super) fn one_item_range<T: Clone>(item: T) -> std::ops::RangeInclusive<T> {
        item.clone()..=item
    }

    pub(super) type ShitResult<'a, T, E> = Result<(T, Input<'a>), E>;
    pub(super) type Range = std::ops::RangeInclusive<Position>;
}
use utils::*;

pub mod function {
    use super::*;

    pub struct Function {
        pub content: Program,
    }

    pub enum Error {}
}
pub use function::Function;

pub mod if_else {
    use super::*;

    pub struct IfElse {
        pub if_branch: Program,
        pub else_branch: Program,
    }

    pub enum Error {}
}
pub use if_else::IfElse;

pub mod shit_loop {
    use super::*;

    pub struct Loop {
        pub content: Program,
    }

    pub enum Error {}
}
pub use shit_loop::Loop;

pub mod name {
    use super::*;

    pub struct Name {
        pub content: String,
    }

    pub(super) fn parse(input: Input) -> ShitResult<Name, ()> {}

    pub struct Error {}
}
pub use name::Name;

pub mod string {
    use super::*;

    pub struct ShitString {
        pub content: String,
    }

    pub(super) fn parse_beginning_marker(input: Input) -> ShitResult<(), ()> {
        parse_char(input, '"')
    }

    pub(super) fn parse(input: Input) -> ShitResult<ShitString, Option<Error>> {
        let mut opening_position = input;
        let mut input = match parse_beginning_marker(input) {
            Err(()) => return Err(None),
            Ok(((), input)) => input,
        };
        while let Some((i, c)) = input.next() {

        }
        Err(Some(Error))
    }

    pub enum Error {
        UnclosedQuote {
            string: 
        }
    }
}
pub use string::ShitString;

pub mod import {
    use super::*;

    pub struct Import {}

    pub enum Error {}
}
pub use import::Import;

pub mod expression {
    use super::*;

    pub enum Expression {
        Function(Function),
        String(ShitString),
        Name(Name),
        IfElse(IfElse),
        Loop(Loop),
        Import(Import),
    }
}
pub use expression::Expression;

pub mod assignment {
    use super::*;

    pub struct Assignment {
        pub name: Name,
        pub value: Expression,
    }

    pub enum AssignmentError {
        MissingName,
        MissingAssignmentSeparator,
        MissingExpression,
    }

    pub(super) fn parse_separator(input: Input) -> ShitResult<(), ()> {
        parse_char(input, '=')
    }

    pub(super) fn parse(input: Input) -> ShitResult<Assignment, Option<AssignmentError>> {}
}
pub use assignment::Assignment;

pub mod statement {
    use super::*;

    pub enum Statement {
        Assignment(Assignment),
        Expression(Expression),
    }
}
pub use statement::Statement;

pub mod program {
    use super::*;

    pub struct Program {
        pub statements: Vec<Statement>,
    }

    pub(super) fn parse_end_marker(input: Input) -> ShitResult<(), ()> {
        parse_known_word(input, "end")
    }

    pub(super) fn parse(input: Input) -> ShitResult<Program, Error<AssignmentError>> {
        let mut assignments = Vec::new();
    }
}
pub use program::Program;

#[derive(Clone)]
pub enum Position {
    ByteOffset(usize),
    EndOfFile,
}

pub struct Error<Kind> {
    pub range: std::ops::RangeInclusive<Position>,
    pub kind: Kind,
}

pub fn parse(input: &str) -> Result<Program, Error<AssignmentError>> {
    Program::parse(Input::new(input)).map(|(program, _rest)| program)
}
