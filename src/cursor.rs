pub const EOF_CHAR: char = '\0';

fn is_whitespace(c: char) -> bool {
    match c {
        | ' ' // space
        | '\n' // linefeed
        | '\r' // carriage return
        | '\t' // horizontal tab
        => true,
        _ => false,
    }
}

pub struct Cursor {
    pub pos: usize,
    chars: Vec<char>,
}

impl Cursor {
    pub fn new(str: &String) -> Cursor {
        Cursor {
            pos: 0,
            chars: str.chars().collect(),
        }
    }

    pub fn nth_char(&self, n: usize) -> char {
        *self.chars.get(n).unwrap_or(&EOF_CHAR)
    }

    pub fn peek(&self) -> char {
        self.nth_char(self.pos)
    }

    pub fn check(&self, c: char) -> bool {
        self.peek() == c
    }

    pub fn next(&mut self) -> char {
        let c = self.peek();
        self.pos += 1;
        c
    }

    pub fn skip_whitespace(&mut self) {
        while is_whitespace(self.peek()) {
            self.next();
        }
    }

    pub fn eat(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.next();
            true
        } else {
            false
        }
    }

    pub fn consume(&mut self, c: char, err: &'static str) -> Result<char, &'static str> {
        if self.peek() == c {
            Ok(self.next())
        } else {
            Err(err)
        }
    }

    pub fn consume_str(&mut self, str: String, err: &'static str) -> Result<String, &'static str> {
        for c in str.chars() {
            if c != self.next() {
                return Err(err);
            }
        }

        Ok(str)
    }
}
