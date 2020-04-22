use std::collections::HashMap;

use crate::ast::Expr;
use crate::cursor::{Cursor, EOF_CHAR};

impl Cursor {
    fn parse_num(&mut self) -> Result<Expr, &'static str> {
        let mut num = vec!();

        if self.peek() == '-' {
            num.push(self.next());
        }

        if self.peek() == '0' {
            num.push(self.next());
        } else {
            let c = self.next();

            if c >= '1' && c <= '9' {
                num.push(c);
            } else {
                return Err("Expected a number")
            }

            while self.peek().is_digit(10) {
                num.push(self.next());
            }
        }

        // Fraction
        if self.peek() == '.' {
            num.push(self.next());

            while self.peek().is_digit(10) {
                num.push(self.next());
            }
        }

        // TODO: Exponent

        let res = num.into_iter().collect::<String>().parse().unwrap();
        Ok(Expr::Num(res))
    }

    fn parse_str(&mut self) -> Result<Expr, &'static str> {
        self.eat('"');

        let mut str = vec!();

        while !self.check('"') {
            let c = self.next();

            if c == '\\' {
                let c = match self.next() {
                    '\"' => '"',
                    '\\' => '\\',
                    '/' => '/',
                    // 'b' => '\b',
                    // 'f' => '\f',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    'u' => {
                        for _ in 0..4 {
                            let c = self.next();

                            if c.is_digit(16) {
                                // TODO: handle char
                            } else {
                                return Err("Expected 4 hex digits");
                            }
                        }

                        'u'
                    },
                    _ => return Err("Unallowed control char"),
                };

                str.push(c);
            } else if c.is_control() {
                return Err("Unallowed control char");
            } else {
                str.push(c);
            }
        }

        self.eat('"');
        Ok(Expr::Str(str.into_iter().collect()))
    }

    fn parse_obj(&mut self) -> Result<Expr, &'static str> {
        self.eat('{');

        let mut obj = HashMap::new();

        while !self.check('}') {
            self.skip_whitespace();
            let k = match self.parse_str()? {
                Expr::Str(k) => k,
                _ => panic!(),
            };

            self.skip_whitespace();
            self.consume(':', "Expected :")?;

            self.skip_whitespace();
            let v = self.parse_expr()?;

            obj.insert(k, v);

            self.skip_whitespace();
            if !self.eat(',') {
                break;
            }
        }

        self.eat('}');
        Ok(Expr::Obj(obj))
    }

    fn parse_arr(&mut self) -> Result<Expr, &'static str> {
        self.eat('[');
        self.skip_whitespace();

        let mut arr = vec!();

        while !self.check(']') {
            let e = self.parse_expr()?;
            arr.push(e);

            self.skip_whitespace();
            if !self.eat(',') {
                break;
            }
        }

        self.eat(']');
        Ok(Expr::Arr(arr))
    }

    fn parse_true(&mut self) -> Result<Expr, &'static str> {
        self.consume_str("true".to_string(), "Expected true")?;
        Ok(Expr::Bool(true))
    }

    fn parse_false(&mut self) -> Result<Expr, &'static str> {
        self.consume_str("false".to_string(), "Expected false")?;
        Ok(Expr::Bool(false))
    }

    fn parse_null(&mut self) -> Result<Expr, &'static str> {
        self.consume_str("null".to_string(), "Expected null")?;
        Ok(Expr::Null)
    }

    fn parse_expr(&mut self) -> Result<Expr, &'static str> {
        self.skip_whitespace();

        match self.peek() {
            c if c.is_digit(10) => self.parse_num(),
            '-' => self.parse_num(),
            '"' => self.parse_str(),
            '{' => self.parse_obj(),
            '[' => self.parse_arr(),
            't' => self.parse_true(),
            'f' => self.parse_false(),
            'n' => self.parse_null(),
            c if c == EOF_CHAR => Err("Unexpected EOL"),
            _ => Err("Unexpected char"),
        }
    }
}

pub fn parse(str: &String) -> Result<Expr, &'static str> {
    Cursor::new(str).parse_expr()
}
