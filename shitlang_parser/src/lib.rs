mod tools;
mod rest;
pub use tools::{SureShit, Rest, char, skip_whitespace, BoolShit, Position};

type ShitResult<'a, T> = Result<tools::SureShit<'a, T>, ParsingError>;

pub enum ParsingError {}

pub enum Value {
    VariableName(String),
    Import(String),
    String(String),
    If(Program, Program),
    Loop(Program),
    Command(Program),
}

pub struct Assignment {
    pub name: String,
    pub value: Value,
}

pub struct Program {
    pub assignments: Vec<Assignment>,
}

const IMPORT: &'static str = "import";
const IF: &'static str = "if";
const ELSE: &'static str = "else";
const END: &'static str = "end";
const LOOP: &'static str = "loop";
const COMMAND: &'static str = "command";
const KEYWORDS: [&'static str; 6] = [IMPORT, IF, ELSE, END, LOOP, COMMAND];
const ASSIGNMENT_SYMBOL: char = '=';
const STRING_BOUNDARY: char = '"';

fn keyword<'a>(mut rest: Rest<'a>, keyword: &str) -> BoolShit<'a> {
    let mut keyword = keyword.chars();
    while let Some((c, new_rest)) = rest.trim() {
        match keyword.next() {
            None => return Some(rest),
            Some(kw_c) => {
                if kw_c != c {
                    return None;
                }
            }
        }
        rest = new_rest;
    }
    None
}

fn varname(mut rest: Rest) -> SureShit<String> {
    let mut name = String::new();
    while let Some((c, new_rest)) = char(rest, |c| {
        !(c.is_whitespace() || [ASSIGNMENT_SYMBOL, STRING_BOUNDARY].contains(&c))
    }) {
        name.push(c);
        rest = new_rest;
    }
    if name.is_empty() || KEYWORDS.contains(&&name[..]) {
        None
    } else {
        name.shrink_to_fit();
        Some((name, rest))
    }
}

pub fn parse(program: &str) -> Result<Program, ParsingError> {
    let rest: Rest = program.into();
}
