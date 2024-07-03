// Parsing results

pub enum ParsingResult<O, I, E> {
    Ok { output: O, rest_of_input: I },
    Err(E),
}

// Input interface

pub enum CuttingError {
    Exhausted(),
}

pub trait Input: Sized {
    type Item;

    fn cut(&self) -> ParsingResult<Self::Item, Self, CuttingError>;
}

impl<T: Iterator + Clone> Input for T {
    type Item = T::Item;

    fn cut(&self) -> ParsingResult<Self::Item, Self, CuttingError> {
        let mut clone = self.clone();
        match clone.next() {
            None => ParsingResult::Err(CuttingError::Exhausted()),
            Some(item) => ParsingResult::Ok {
                output: item,
                rest_of_input: clone,
            },
        }
    }
}

// Parser interface

pub trait Parser<O, I, NI, E> {
    fn parse(&self, input: &I) -> ParsingResult<O, NI, E>;
}

impl<T: Fn(&I) -> ParsingResult<O, NI, E>, O, I, NI, E> Parser<O, I, NI, E> for T {
    fn parse(&self, input: &I) -> ParsingResult<O, NI, E> {
        self(input)
    }
}

// Combinators

pub fn filter<I: Input>(f: impl Fn(&I::Item) -> bool) -> impl Parser<I::Item, I, I, ()> {
    move |input: &I| match input.cut() {
        ParsingResult::Ok {
            output,
            rest_of_input,
        } => {
            if f(&output) {
                ParsingResult::Ok {
                    output,
                    rest_of_input,
                }
            } else {
                ParsingResult::Err(())
            }
        }
        ParsingResult::Err(_cutting_error) => ParsingResult::Err(()),
    }
}

pub fn matching<Item: Eq, I: Input<Item = Item>>(
    pattern: I::Item,
) -> impl Parser<I::Item, I, I, ()> {
    filter(move |item| pattern.eq(item))
}

pub fn any<I: Input>() -> impl Parser<I::Item, I, I, ()> {
    filter(|_item| true)
}

pub struct BranchingParser<P, B, P2, B2> {
    previous: Option<BranchingParser<P, B>>,
    predicate: P2,
    branch: B2,
}

impl<P, B, P2, B2> BranchingParser<P, B, P2, B2> {
    pub fn else_if<P3, B3>(self, predicate: P3, branch: B3) -> BranchingParser<>
    pub fn else_if<P2: Parser<>, B2: Parser<>>(self, predicate: P2, branch: B2) -> BranchingParser<>
}

impl<Q, I, E> BranchingParser<Q, I, E> {
    pub fn else_<P: Parser>(self, branch: P) -> impl Parser<> {}

    pub fn else_if<P: Parser, Q: Parser>(self, query: Q, branch: P) -> BranchingParser<Q, > {

    }
}

pub fn if_(query: impl Parser<>, branch: Fn(...) -> impl Parser<>) -> BranchingParser<> {

}
