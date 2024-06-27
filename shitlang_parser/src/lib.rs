/*
struct Pos {
    pub col: usize,
    pub row: usize,
}

struct Before<'a> {
    pub s: &'a str,
    pub beg: Pos,
    pub end: Pos,
}

struct After<'a> {
    pub s: &'a str,
    pub beg: Pos,
}
*/

struct Roller<'a> {
    s: &'a str,
    idx: usize,
}

struct Roll<'a> {
    roller: Roller<'a>,
    c: char,
}

impl<'a> Roll<'a> {
    fn before_idx(&self, idx: usize) -> &'a str {
        if cfg!(debug_assertions) {
            &self.roller.s[0..idx]
        } else {
            unsafe { self.roller.s.get_unchecked(0..idx) }
        }
    }

    fn after_idx(&self, idx: usize) -> &'a str {
        if cfg!(debug_assertions) {
            &self.roller.s[idx..]
        } else {
            unsafe { self.roller.s.get_unchecked(idx..) }
        }
    }

    pub fn after(&self) -> &'a str {
        self.after_idx(self.roller.idx)
    }

    pub fn before(&self) -> &'a str {
        self.before_idx(self.roller.idx - self.c.len_utf8())
    }

    pub fn before_inclusive(&self) -> &'a str {
        self.before_idx(self.roller.idx)
    }

    pub fn after_inclusive(&self) -> &'a str {
        self.after_idx(self.roller.idx - self.c.len_utf8())
    }
}

impl<'a> Iterator for Roller<'a> {
    type Item = Roll<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.after().chars();
        chars.next().inspect(|c| {
            self.idx += c.len_utf8();
        })
    }
}

fn roll(s: &str) -> Roller {
    Roller { s, idx: 0 }
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
