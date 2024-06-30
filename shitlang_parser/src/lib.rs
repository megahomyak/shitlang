mod different_approach;

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
        if string::parse_opening_marker(input).is_err()
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

    pub(super) type ShitResult<'a, T, E> = Result<(T, Input<'a>), E>;
    pub(super) type Span = std::ops::RangeInclusive<Position>;
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

    pub(super) fn parse(mut input: Input) -> ShitResult<Name, ()> {
        if import::parse_opening_marker(input).is_err()
            && function::parse_opening_marker(input).is_err()
            && string::parse_opening_marker(input).is_err()
            && shit_loop::parse_opening_marker(input).is_err()
            && if_else::parse_opening_marker(input).is_err()
            && if_else::parse_branch_separator(input).is_err()
            && parse_end_marker(input).is_err()
        {
            let mut content = String::new();
            while let Ok((c, new_input)) = parse_word_char(input) {
                content.push(c);
                input = new_input;
            }
            if !content.is_empty() {
                content.shrink_to_fit();
                return Ok((Name { content }, input));
            }
        }
        Err(())
    }

    pub struct Error {}
}
pub use name::Name;

pub mod string {
    use super::*;

    pub struct ShitString {
        pub content: String,
    }

    pub(super) fn parse_opening_marker(input: Input) -> ShitResult<(), ()> {
        match parse_char(input, '"') {
            Err(()) => Err(()),
            Ok(((), input)) => Ok(((), input)),
        }
    }

    pub(super) fn parse(mut input: Input) -> ShitResult<ShitString, Option<Error>> {
        let opening_index = match input.next() {
            None => return Err(None),
            Some((i, _c)) => i,
        };
        let mut input = match parse_opening_marker(input) {
            Err(()) => return Err(None),
            Ok(((), input)) => input,
        };
        let mut content = String::new();
        while let Some((i, c)) = input.next() {
            match c {
                '\\' => {
                    let sequence_beginning_index = i;
                    match input.next() {
                        None => {
                            return Err(Some(Error::EscapeCharacterAtEndOfInput {
                                escape_sequence_span: Position::ByteOffset(sequence_beginning_index)
                                    ..=Position::EndOfFile,
                            }));
                        }
                        Some((_i, c @ ('\\' | '"'))) => content.push(c),
                        Some((i, _c)) => {
                            return Err(Some(Error::UnknownCharacterEscaped {
                                escape_sequence_span: Position::ByteOffset(sequence_beginning_index)
                                    ..=Position::ByteOffset(i),
                            }));
                        }
                    }
                }
                '"' => {
                    content.shrink_to_fit();
                    return Ok((ShitString { content }, input));
                }
                c => content.push(c),
            }
        }
        Err(Some(Error::UnclosedQuote {
            string_span: Position::ByteOffset(opening_index)..=Position::EndOfFile,
        }))
    }

    pub enum Error {
        UnclosedQuote { string_span: Span },
        EscapeCharacterAtEndOfInput { escape_sequence_span: Span },
        UnknownCharacterEscaped { escape_sequence_span: Span },
    }
}
pub use string::ShitString;

pub mod import {
    use super::*;

    pub(super) fn parse(input: Input) -> ShitResult<Import, Option<Error>> {
        let input = match parse_opening_marker(input) {
            Err(()) => return Err(None),
            Ok(((), input)) => input,
        };
        let input = skip_whitespace(input);
        let (string, input) = match string::parse(input) {
            Ok((string, input)) => (string, input),
            Err(Some(error)) => return Err(Some(Error::))
        };
        Ok((Import { file_path }, input))
    }

    pub(super) fn parse_opening_marker(input: Input) -> ShitResult<(), ()> {
        parse_known_word(input, "import")
    }

    pub struct Import {
        pub file_path: ShitString,
    }

    pub enum Error {
        
    }
}
pub use import::Import;

pub mod expression {
    use super::*;

    pub enum Expression {
        Function(Function),
        String(ShitString),
        Name(Name, Span),
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

    pub enum Error {
        MissingName,
        MissingAssignmentSeparator,
        MissingExpression,
    }

    pub(super) fn parse_separator(input: Input) -> ShitResult<(), ()> {
        parse_char(input, '=')
    }

    pub(super) fn parse(input: Input) -> ShitResult<Assignment, Option<Error>> {}
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

    pub(super) fn parse(input: Input) -> ShitResult<Program, Error<AssignmentError>> {
        let mut assignments = Vec::new();
    }
}
pub use program::Program;

fn parse_end_marker(input: Input) -> ShitResult<(), ()> {
    parse_known_word(input, "end")
}

#[derive(Clone)]
pub enum Position {
    ByteOffset(usize),
    EndOfFile,
}

pub fn parse(input: &str) -> Result<Program, Error<AssignmentError>> {
    Program::parse(Input::new(input)).map(|(program, _rest)| program)
}
