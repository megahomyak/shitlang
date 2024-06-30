type Input<'a> = std::str::CharIndices<'a>;

#[derive(Clone)]
pub enum Position {
    ByteOffset(usize),
    EndOfFile,
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

fn parse_string_delimiter(c: char, input: Input) -> bool {

}

fn parse_assignment_sign(c: char, input: Input) -> ShitResult<(), ()> {
    parse_char(input, '=')
}

const END: &str = "end";

fn any_matches<const N: usize>(c: char, input: Input, parsers: [fn (char, Input) -> bool; N]) -> bool {
    for parser in &parsers {
        if parser(c, input.clone()) {
            return false;
        }
    }
    true
}

fn parse_word_char(mut input: Input) -> ShitResult<char, ()> {
    let mut initial_input = input;
    match input.next() {
        None => (),
        Some((i, c)) => {
            if !c.is_whitespace() {
                return Ok((c, ))
            }
        }
    }
    Err(())
    if any_matches(initial_input.clone(), [parse_string_delimiter, parse_assignment_sign]) || c.is_whitespace() {
        Err(())
    } else {
        Ok((input.next(), input))
    }
}

fn is_at_word_boundary(current_character: char, input: Input) -> bool {
}

fn parse_known_word(mut input: Input, word: &str) -> Option<Input> {
    for c in word.chars() {
        input = parse_char(input, c)?;
    }
    match input.peek() {
        None => Some(input),
        Some((i, c)) => {
            if is_at_word_boundary(c, input) {}
            input.advance();
        }
    }
}

fn parse_unknown_word(mut input: Input) -> ShitResult<String, ()> {
    let mut name = String::new();
    while let Some((_i, c)) = input.peek() {
        if any_matches(input.clone(), [parse_string_delimiter, parse_assignment_sign]) || c.is_whitespace() {
            break;
        }
        name.push(c);
        input.advance();
    }
    if name.is_empty() {
        Err(())
    } else {
        name.shrink_to_fit();
        Ok((name, input))
    }
}

impl Name {
    fn parse()
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
    MissingAssignmentSign,
    MissingExpression,
}

fn parse_keyword(input: Input, keyword: &'static str) -> Option<Input> {

}

impl Assignment {
    fn parse(input: Input) -> ShitResult<Self, Option<AssignmentError>> {
        let input = skip_whitespace(input);
        let (var_name, input) = match Name::parse(input.clone()) {
            Ok((var_name, input)) =>
            Err(()) => if 
        }
        let Ok((var_name, input)) = Name::parse(input.clone()) else {
            return Err(Error {
                range: one_item_range(match input.peek() {
                    Some((i, _c)) => Position::ByteOffset(i),
                    None => Position::EndOfFile,
                }),
                kind: AssignmentError::MissingName,
            });
        };
        let input = skip_whitespace(input);
        match input.peek() {
            None => Err(Error {
                range: one_item_range(Position::EndOfFile),
                kind: ProgramError::
            }),
        }
    }
}

pub struct Program {
    pub assignments: Vec<Assignment>,
}

pub struct Error<Kind> {
    pub range: std::ops::RangeInclusive<Position>,
    pub kind: Kind,
}

type ShitResult<'a, T, E> = Result<(T, Input<'a>), E>;

fn skip_whitespace(mut input: Input) -> Input {
    while let Some((_i, c)) = input.peek() {
        if !c.is_whitespace() {
            break;
        }
        input = input.advanced();
    }
    input
}

fn one_item_range<T: Clone>(item: T) -> std::ops::RangeInclusive<T> {
    item.clone()..=item
}

impl Program {
    fn parse(input: Input) -> ShitResult<Program, Error<AssignmentError>> {
        let mut assignments = Vec::new();
        loop {

        }
    }
}

pub fn parse(input: &str) -> Result<Program, Error<AssignmentError>> {
    Program::parse(Input::new(input)).map(|(program, _rest)| program)
}
