pub struct Offset(usize);
pub struct Positioned<T>(Offset, T);

pub struct Error {

}

type Result<T> = std::iter::Peekable<std::result::Result<Positioned<T>, Positioned<Error>>>;

struct Program {

}

fn parse(i: Input) -> Result<Program> {}
