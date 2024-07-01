// Parsing results

pub enum ParsingResult<T, I, E> {
    Ok(T, I),
    Err(E),
}

pub enum ParsingError<E> {
    NotRecognized(),
    Invalid(E),
}

pub struct NotRecognized();

// Input interface

pub trait Input: Sized {
    type Item;
    type Parser: Parser<Self::Item, Self, NotRecognized>;

    fn cut(&self) -> Self::Parser;
}

pub struct IterCutter();

impl<I> Parser<I::Item, I, NotRecognized> for IterCutter
where
    I: Iterator + Clone,
{
    fn parse(&self, input: I) -> ParsingResult<I::Item, I, NotRecognized> {
        let mut clone = input.clone();
        match clone.next() {
            None => ParsingResult::Err(NotRecognized()),
            Some(item) => ParsingResult::Ok(item, clone),
        }
    }
}

impl<T> Input for T
where
    T: Iterator + Clone,
{
    type Item = T::Item;
    type Parser = IterCutter;

    fn cut(&self) -> Self::Parser {
        IterCutter()
    }
}

/*
impl<T> Input for T
where
    T: Iterator + Clone,
{
    type Item = <T as Iterator>::Item;

    fn cut(&self) -> ParsingResult<Self::Item, Self, NotRecognized> {
        let mut clone = self.clone();
        match clone.next() {
            None => ParsingResult::Err(NotRecognized()),
            Some(item) => ParsingResult::Ok(item, clone),
        }
    }
}
*/

// Parser interface

pub trait Parser<T, I, E> {
    fn parse(&self, input: I) -> ParsingResult<T, I, E>;
}

/*

impl<T, O, I, E> Parser<O, I, E> for T
where
    T: Fn(I) -> ParsingResult<O, I, E>,
{
    fn parse(&self, input: I) -> ParsingResult<O, I, E> {
        self(input)
    }
}

pub fn cut<I, C>(checker: C) -> impl Parser<I::Item, I, NotRecognized>
where
    I: Input,
    C: Fn(&I::Item) -> bool,
{
    |input: I| {
        input.cut().then(|c, input| {
            if checker(&c) {
                ParsingResult::Ok(c, input)
            } else {
                ParsingResult::Err(NotRecognized())
            }
        })
    }
}

pub fn parse_repeating<T, I, C, E, P>(
    mut collection: C,
    input: I,
    parser: P,
) -> ParsingResult<C, I, E>
where
    I: Iterator<Item = T> + Clone,
    C: Extend<T>,
    P: Fn(I) -> ParsingResult<T, I, Option<E>>,
{
    struct Collector<P, I, E> {
        parser: P,
        input: I,
        error: Option<E>,
    }

    impl<T, P, I, E> Iterator for Collector<P, I, E>
    where
        P: Fn(I) -> ParsingResult<T, I, Option<E>>,
        I: Clone,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            match (self.parser)(self.input.clone()) {
                ParsingResult::Ok(t, rest) => {
                    self.input = rest;
                    Some(t)
                }
                ParsingResult::Err(None) => None,
                ParsingResult::Err(Some(e)) => {
                    self.error = Some(e);
                    None
                }
            }
        }
    }

    let mut collector = Collector {
        input,
        parser,
        error: None,
    };
    collection.extend(&mut collector);
    match collector.error {
        Some(e) => ParsingResult::Err(e),
        None => ParsingResult::Ok(collection, collector.input),
    }
}
*/
