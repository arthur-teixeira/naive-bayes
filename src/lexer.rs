pub struct Lexer<'a> {
    pub query: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(query: &'a [char]) -> Self {
        Lexer { query }
    }

    fn trim(&mut self) {
        while !self.query.is_empty() && self.query[0].is_whitespace() {
            self.query = &self.query[1..];
        }
    }

    fn chop_by(&mut self, predicate: fn(char) -> bool) -> String {
        let mut i = 0;
        while i < self.query.len() && predicate(self.query[i]) {
            i += 1;
        }

        let ret = &self.query[0..i];
        self.query = &self.query[i..];

        return ret.iter().map(|x| x.to_ascii_lowercase()).collect();
    }

    pub fn next_token(&mut self) -> Option<String> {
        self.trim();

        if self.query.is_empty() {
            return None;
        }

        if self.query[0].is_alphabetic() {
            return Some(self.chop_by(|x| x.is_alphabetic()));
        };

        if self.query[0].is_numeric() {
            return Some(self.chop_by(|x| x.is_numeric()));
        };

        let ret = &self.query[0..1];
        self.query = &self.query[1..];
        return Some(ret.iter().collect());
    }
}

impl Iterator for Lexer<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

