pub enum ParsingResult<T, I, E> {
    Ok(T, I),
    Err(E),
}

impl<T, I, E> ParsingResult<T, I, E> {
    pub fn and<NT, NI>(
        self,
        f: impl Fn(T, I) -> ParsingResult<NT, NI, E>,
    ) -> ParsingResult<NT, NI, E> {
        match self {
            Self::Ok(t, input) => f(t, input),
            Self::Err(e) => ParsingResult::Err(e),
        }
    }

    pub fn map<NT>(self, f: impl Fn(T) -> NT) -> ParsingResult<NT, I, E> {
        match self {
            Self::Ok(t, input) => ParsingResult::Ok(f(t), input),
            Self::Err(e) => ParsingResult::Err(e),
        }
    }
}

impl<T, I, E> ParsingResult<T, I, Option<E>> {
    pub fn or(self, f: impl Fn() -> Self) -> Self {
        match self {
            Self::Ok(..) => self,
            Self::Err(None) => f(),
            Self::Err(Some(..)) => self,
        }
    }
}

pub fn parse_matching<T, I: Iterator, E>(
    mut input: I,
    checker: impl Fn(I::Item) -> Option<T>,
) -> ParsingResult<T, I, Option<E>> {
    match input.next() {
        None => ParsingResult::Err(None),
        Some(item) => match checker(item) {
            None => ParsingResult::Err(None),
            Some(result) => ParsingResult::Ok(result, input),
        },
    }
}
