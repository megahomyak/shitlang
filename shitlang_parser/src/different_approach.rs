// Utilities

type Input<'a> = std::str::CharIndices<'a>;

type ShitResult<'a, T, E> = parser_combinators::ParsingResult<T, Input<'a>, E>;
type ParsingError = parser_combinators::ParsingError<Error>;
use parser_combinators::NotRecognized;

type Span = std::ops::RangeInclusive<Position>;

fn cut_any(input: Input) -> ShitResult<char, NotRecognized> {

}

fn cut(input: Input, pattern: char) -> ShitResult<char, NotRecognized> {
    parser_combinators::parse_one_matching(input, |(_i, c)| (c == pattern).then(|| c))
}

fn cut_not(input: Input, pattern: char) -> ShitResult<char, NotRecognized> {
    parser_combinators::parse_one_matching(input, |(_i, c)| (c != pattern).then(|| c))
}

fn err<'a, T>(error: Error) -> ShitResult<'a, T, ParsingError> {
    ShitResult::Err(ParsingError::Invalid(error))
}

fn not_recognized<'a, T>() -> ShitResult<'a, T, ParsingError> {
    ShitResult::Err(ParsingError::NotRecognized())
}

fn ok<T, E>(t: T, input: Input) -> ShitResult<T, E> {
    ShitResult::Ok(t, input)
}

// Output

pub enum Position {
    EndOfInput,
    ByteOffset(usize),
}
use Position::{ByteOffset, EndOfInput};

pub enum Error {
    NoCharacterAfterEscapeCharacterInString { escape_sequence_span: Span },
    UnexpectedCharacterEscapedInString { escape_sequence_span: Span },
}

pub struct Whitespace();
pub struct EndingMark();

pub struct IfBeginningMark();
pub struct IfProgram(Program);
pub struct ElseBeginningMark();
pub struct ElseProgram(Program);
pub struct IfElse(IfBeginningMark, IfProgram, ElseBeginningMark, ElseProgram);

pub struct LoopBeginningMark();
pub struct LoopBody(Program);
pub struct LoopEndingMark();
pub struct Loop(LoopBeginningMark, LoopBody, EndingMark);

pub struct EscapedStringContentChar(char);
fn parse_escaped_string_content_char(input: Input) -> ShitResult<EscapedStringContentChar, ParsingError> {
    cut(input, '\\').then(|_, input| {
        cut(input, '\\').then(|c, input| ok(c, input))
            .or(|| cut(input, '"').then(|c, input| ok(c, input)))
            .or(|| )
    })
}
pub struct UnescapedStringContentChar(char);
fn parse_unescaped_string_content_char(input: Input) -> ShitResult {}
pub enum StringContentChar {
    EscapedStringContentChar(EscapedStringContentChar),
    UnescapedStringContentChar(UnescapedStringContentChar),
}
fn parse_string_content_char(input: Input) -> ShitResult<StringContentChar, ParsingError> {
    match input.next() {
        None | Some((_, '"')) => not_recognized(),
        Some((span_beginning, '\\')) => match input.next() {
            None => err(Error::NoCharacterAfterEscapeCharacterInString {
                escape_sequence_span: ByteOffset(span_beginning)..=EndOfInput,
            }),
            Some((_, same @ '\\')) | Some((_, same @ '"')) => ok(same, input),
            Some((span_end, c)) => err(Error::UnexpectedCharacterEscapedInString {
                escape_sequence_span: ByteOffset(span_beginning)..=ByteOffset(span_end),
            }),
        },
        Some(c) => ok(StringContentChar(c), input),
    }
}
pub struct StringContent(Vec<StringContentChar>);
fn parse_string_content(input: Input) -> ShitResult<StringContent, Option<Error>> {}
pub struct StringDelimiter();
fn parse_string_delimiter(input: Input) -> ShitResult<StringDelimiter, ()> {
    parse_char(input, '"').map_ok(|()| StringDelimiter)
}
pub struct ShitString(StringDelimiter, StringContent, StringDelimiter);

pub struct ImportBeginningMark();
pub struct ImportFilePath(ShitString);
pub struct Import(ImportBeginningMark, ImportFilePath);

pub struct FunctionBeginningMark();
pub struct Function(FunctionBeginningMark, EndingMark);

pub struct NameChar(char);
pub struct Name(Vec<NameChar>);

pub enum Expression {
    Import(Import),
    Name(Name),
    ShitString(ShitString),
    Function(Function),
    Loop(Loop),
    IfElse(IfElse),
}

pub struct AssignmentOperator();
fn parse_assignment_operator(input: Input) -> ShitResult<AssignmentOperator, ()> {
    parse_char(input, '=').map_ok(|()| AssignmentOperator)
}

pub struct Assignment(Name, AssignmentOperator, Expression);

pub enum Statement {
    Expression(Expression),
    Assignment(Assignment),
}

pub struct Program(Vec<Statement>);
