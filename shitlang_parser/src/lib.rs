// Utilities

type Input<'a> = std::str::CharIndices<'a>;

trait ShitParser<'a, O, E>: parser_combinators::Parser<O, Input<'a>, Input<'a>, E> {}
impl<'a, T: parser_combinators::Parser<O, Input<'a>, Input<'a>, E>, O, E> ShitParser<'a, O, E>
    for T
{
}

enum ParsingError {
    Recoverable(),
    Unrecoverable(Error),
}
use ParsingError::{Recoverable, Unrecoverable};

type Span = std::ops::RangeInclusive<Position>;

use parser_combinators::cut;
use parser_combinators::ParserExt;
use parser_combinators::PredicateCuttingError::{self, NotMatched};

fn cut_exact<'a>(
    pattern: char,
) -> impl ShitParser<'a, <Input<'a> as Iterator>::Item, PredicateCuttingError> {
    cut(move |(_i, c)| *c == pattern)
}

fn cut_any<'a>() -> impl ShitParser<'a, <Input<'a> as Iterator>::Item, PredicateCuttingError> {
    cut(|_| true)
}

fn unrec<O>(e: Error) -> impl ShitParser<'a, O, ParsingError> {
    |input| parser_combinators::ParsingResult::Err(ParsingError::Unrecoverable(e))
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
fn parse_escaped_string_content_char<'a>(
) -> impl ShitParser<'a, EscapedStringContentChar, ParsingError> {
    /*
    if '\\' {
        if Some('\\') {
            ok('\\')
        } else if Some('"') {
            ok('"')
        } else if Some(_) {
            err(UnexpectedCharacterAfterEscape)
        } else if None {
            err(NoCharacterAfterEscape)
        }
    } else {
      not_matched()
    }

    span(
        if_(matches('\\'),
            if_(matches('\\'),
                ok('\\')
            ).else_if(matches('"'),
                ok('"')
            ).else_if(any,
                err(UnexpectedCharacterAfterEscape)
            ).else_(
                err(NoCharacterAfterEscape)
            )
        ).else_(
            not_matched()
        )
    )
     */
    cut('\\').and(record(|(span_beginning, _c)|, cut('\\').or(cut('"')).map(|| EscapedStringContentChar)))
    cut_exact('\\')
        .map_err(|NotMatched()| Recoverable())
        .then(|(span_beginning, _c)| {
            cut_exact('\\')
                .or(|NotMatched()| cut_exact('"'))
                .map(|(_i, c)| EscapedStringContentChar(c))
                .or(|NotMatched()| {
                    cut_any()
                        .map_err(|NotMatched()| {
                            Unrecoverable(Error::NoCharacterAfterEscapeCharacterInString {
                                escape_sequence_span: ByteOffset(span_beginning)..=EndOfInput,
                            })
                        })
                        .then(|(span_end, _c)| {
                            |_input| {
                                parser_combinators::ParsingResult::Err(ParsingError::Unrecoverable(
                                    Error::UnexpectedCharacterEscapedInString {
                                        escape_sequence_span: ByteOffset(span_beginning)
                                            ..=ByteOffset(span_end),
                                    },
                                ))
                            }
                        })
                })
        })
}
pub struct UnescapedStringContentChar(char);
pub enum StringContentChar {
    EscapedStringContentChar(EscapedStringContentChar),
    UnescapedStringContentChar(UnescapedStringContentChar),
}
pub struct StringContent(Vec<StringContentChar>);
pub struct StringDelimiter();
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

pub struct Assignment(Name, AssignmentOperator, Expression);

pub enum Statement {
    Expression(Expression),
    Assignment(Assignment),
}

pub struct Program(Vec<Statement>);
