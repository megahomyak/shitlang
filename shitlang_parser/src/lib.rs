mod pos;
use pos::Pos;

struct Positioned<T> {
    pub pos: Pos,
    pub content: T,
}

enum Expression {

}

struct Assignment {
    pub name: String,
    pub value: Positioned<Expression>,
}

struct Program {
    pub assignments: Vec<Positioned<Assignment>>,
}

struct Positioned<T> {
    pub pos: pos::Pos,
    pub contents: T,
}

pub fn parse() -> Positioned<Program> {

}
