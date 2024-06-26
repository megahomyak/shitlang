mod rest {
    pub struct Rest<'a> {
        s: &'a str,
        pos: Pos,
    }

    pub struct Pos {
        pub col: usize,
        pub row: usize,
    }

    impl Rest<'_> {
        pub fn trim(&mut self) -> Option<char> {
            let mut chars = self.s.chars();
            chars.next().inspect(|c| {
                if *c == '\n' {
                    self.pos.row += 1;
                    self.pos.col = 0;
                } else {
                    self.pos.col += 1;
                }
            })
        }
    }
}
use rest::Rest;

/// Distinguish string literals, as they're the most pervasive
mod stage_1 {
    use super::*;

    pub enum Token {

    }

    fn parse(rest: Rest) -> Vec<Token> {
        while let Some(c) = rest.trim() {

        }
    }
}
