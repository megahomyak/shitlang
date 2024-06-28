mod pos {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Pos {
        col: usize,
        row: usize,
    }

    const FIRST_COL: usize = 0;

    impl Pos {
        pub fn new() -> Self {
            Self {
                col: FIRST_COL,
                row: 1,
            }
        }

        pub fn advance(&mut self, c: char) {
            if c == '\n' {
                self.row += 1;
                self.col = FIRST_COL;
            } else {
                self.col += 1;
            }
        }

        pub fn advanced(s: impl Iterator<Item = char>) -> Self {
            let mut new = Self::new();
            for c in s {
                new.advance(c);
            }
            new
        }

        pub fn col(&self) -> usize {
            self.col
        }

        pub fn row(&self) -> usize {
            self.row
        }
    }
}
pub use pos::Pos;

mod slice {
    use super::*;

    pub struct Slice<'a> {
        s: &'a str,
        beg: Pos,
        end: Pos,
    }

    impl<'a> From<&'a str> for Slice<'a> {
        fn from(s: &'a str) -> Self {
            Self {
                s,
                beg: Pos::new(),
                end: Pos::advanced(s.chars()),
            }
        }
    }

    impl<'a> Slice<'a> {
        pub fn iter(&'a self) -> Iter<'a> {
            Iter {
                slice: self,
                cur: self.beg,
                idx: 0,
            }
        }
    }

    pub struct Iter<'a> {
        slice: &'a Slice<'a>,
        cur: Pos,
        idx: usize,
    }

    impl<'a> Iter<'a> {
        pub fn before(&self) -> Slice<'a> {
            let s = unsafe { self.slice.s.get_unchecked(..self.idx) };
            Slice {
                s,
                beg: self.slice.beg,
                end: self.cur,
            }
        }

        pub fn after(&self) -> Slice<'a> {
            let s = unsafe { self.slice.s.get_unchecked(self.idx..) };
            Slice {
                s,
                beg: self.slice.beg,
                end: self.cur,
            }
        }
    }

    impl<'a> Iterator for Iter<'a> {
        type Item = char;

        fn next(&mut self) -> Option<Self::Item> {
            self.after().s.chars().next().inspect(|c| {
                self.cur.advance(*c);
                self.idx += c.len_utf8();
            })
        }
    }
}
use slice::Slice;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let mut iter: slice::Iter = Slice::from("abc").iter();
        assert_eq!(iter.next(), Some('a'));
        assert_eq!(iter.next(), Some('b'));
        assert_eq!(iter.next(), Some('c'));
        assert_eq!(
            iter.before(),
            Slice {
                s: "abc",
                beg: Pos::new(),
            }
        );
        assert_eq!(iter.after(), "def");
        assert_eq!(iter.next(), Some('d'));
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
