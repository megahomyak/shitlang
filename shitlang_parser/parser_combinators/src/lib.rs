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

// Parser extensions

pub trait ParserExt<O, I, NI, E>: Parser<O, I, NI, E> {
    fn then<NO, NNI, P: Parser<NO, NI, NNI, E>>(
        &self,
        f: impl Fn(O) -> P,
    ) -> impl Parser<NO, I, NNI, E>;

    fn or<NE, P: Parser<O, I, NI, NE>>(&self, f: impl Fn(E) -> P) -> impl Parser<O, I, NI, NE>;

    fn map<NO>(&self, f: impl Fn(O) -> NO) -> impl Parser<NO, I, NI, E>;

    fn map_err<NE>(&self, f: impl Fn(E) -> NE) -> impl Parser<O, I, NI, NE>;
}

impl<T: Parser<O, I, NI, E>, O, I, NI, E> ParserExt<O, I, NI, E> for T {
    fn then<NO, NNI, P: Parser<NO, NI, NNI, E>>(
        &self,
        f: impl Fn(O) -> P,
    ) -> impl Parser<NO, I, NNI, E> {
        move |input: &I| match self.parse(input) {
            ParsingResult::Err(e) => ParsingResult::Err(e),
            ParsingResult::Ok {
                output,
                rest_of_input,
            } => f(output).parse(&rest_of_input),
        }
    }

    fn or<NE, P: Parser<O, I, NI, NE>>(&self, f: impl Fn(E) -> P) -> impl Parser<O, I, NI, NE> {
        move |input: &I| match self.parse(input) {
            ParsingResult::Ok {
                output,
                rest_of_input,
            } => ParsingResult::Ok {
                output,
                rest_of_input,
            },
            ParsingResult::Err(e) => f(e).parse(input),
        }
    }

    fn map<NO>(&self, f: impl Fn(O) -> NO) -> impl Parser<NO, I, NI, E> {
        move |input: &I| match self.parse(input) {
            ParsingResult::Err(e) => ParsingResult::Err(e),
            ParsingResult::Ok {
                output,
                rest_of_input,
            } => ParsingResult::Ok {
                output: f(output),
                rest_of_input,
            },
        }
    }

    fn map_err<NE>(&self, f: impl Fn(E) -> NE) -> impl Parser<O, I, NI, NE> {
        move |input: &I| match self.parse(input) {
            ParsingResult::Err(e) => ParsingResult::Err(f(e)),
            ParsingResult::Ok {
                output,
                rest_of_input,
            } => ParsingResult::Ok {
                output,
                rest_of_input,
            },
        }
    }
}

// Combinators

pub enum PredicateCuttingError {
    NotMatched(),
}

pub fn cut<I: Input>(
    f: impl Fn(&I::Item) -> bool,
) -> impl Parser<I::Item, I, I, PredicateCuttingError> {
    move |input: &I| {
        if let ParsingResult::Ok {
            output,
            rest_of_input,
        } = input.cut()
        {
            if f(&output) {
                return ParsingResult::Ok {
                    output,
                    rest_of_input,
                };
            }
        }
        ParsingResult::Err(PredicateCuttingError::NotMatched())
    }
}
