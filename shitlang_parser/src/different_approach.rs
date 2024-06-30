type Input<'a> = std::str::CharIndices<'a>;

pub enum Error {}

type ShitResult = pars

pub struct EndingMark;
fn parse_ending_mark(input: Input) -> ShitResult<EndingMark> {
    parse_char(input, '"').map(|()| EndingMark)
}

pub struct IfBranch(Program);
pub struct ElseBranch(Program);
pub struct IfElse(IfBranch, ElseBranch);

pub struct LoopBeginningMark;
pub struct LoopBody(Program);
pub struct LoopEndingMark;
pub struct Loop(LoopBeginningMark, LoopBody, EndingMark);

pub struct StringContent(String);
fn parse_string_content(input: Input) -> ShitResult<StringContent> {
    input.next()
}
pub struct StringDelimiter(String);
pub struct ShitString(StringDelimiter, StringContent, StringDelimiter);

pub struct ImportBeginningMark;
pub struct ImportFilePath(ShitString);
pub struct Import(ImportBeginningMark, ImportFilePath);

pub struct FunctionBeginningMark;
pub struct Function(FunctionBeginningMark, EndingMark);

pub struct Name(String);

pub enum Expression {
    Import(Import),
    Name(Name),
    ShitString(ShitString),
    Function(Function),
    Loop(Loop),
    IfElse(IfElse),
}

pub struct AssignmentOperator;
fn parse_assignment_operator(input: Input) -> Result<ShitResult> {}

pub struct Assignment(Name, AssignmentOperator, Expression);

pub enum Statement {
    Expression(Expression),
    Assignment(Assignment),
}

pub struct Program(Vec<Statement>);
