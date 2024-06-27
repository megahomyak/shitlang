struct Pos {
    col: usize,
    row: usize,
}

/*
#[rustfmt::skip]
impl Pos {
    fn col(&self) -> usize { self.col }
    fn row(&self) -> usize { self.row }
}
*/

struct Seen<'a> {
    s: &'a str,
    beg: Pos,
    end: Pos,
}

struct Unseen<'a> {
    s: &'a str,
    beg: Pos,
}

trait Slice<'a>: Sized {
    type After;

    fn iter(&self) -> Iter<Self>;

    unsafe fn split(&self, pos: Pos, idx: usize) -> (Seen<'a>, Self::After);
}

struct Iter<'a, S: Slice<'a>> {
    slice: S,
    idx: usize,
    cur: Pos,
}

struct Char<I> {

}

impl<S: Slice> Iterator for Iter<S> {
    type Item = (Seen, char, S::After);

    fn next(&mut self) -> Option<Self::Item> {

    }
}

impl<'a> Iterator for Seen<'a> {
    type Item = (Seen, char, Seen);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling() {
        let mut roller = roll("abcdef");
        assert_eq!(roller.next(), Some('a'));
        assert_eq!(roller.next(), Some('b'));
        assert_eq!(roller.next(), Some('c'));
        assert_eq!(roller.before(), "abc");
        assert_eq!(roller.after(), "def");
        assert_eq!(roller.next(), Some('d'));
        assert_eq!(roller.next(), Some('e'));
        assert_eq!(roller.next(), Some('f'));
        assert_eq!(roller.next(), None);
    }

    #[test]
    fn test_parsing() {
        let mut roller = roll("abc\"def\"ghi");
        while let Some(c) = roller.next() {
            if c == '"' {}
        }
    }
}
