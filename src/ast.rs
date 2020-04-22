use std::fmt;
use std::collections::HashMap;

pub enum Expr {
    Num(f64),
    Str(String),
    Obj(HashMap<String, Expr>),
    Arr(Vec<Expr>),
    Bool(bool),
    Null,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Num(v) => v.fmt(f),
            Expr::Str(v) => write!(f, "\"{}\"", v),
            Expr::Obj(v) => {
                let mut str = v.iter()
                    .map(|(k, v)| format!(r#""{}":{}"#, k, v.to_string()))
                    .fold(String::new(), |acc, x| acc + &x + ",");
                str.pop();

                write!(f, "{{{}}}", str)
            },
            Expr::Arr(v) => {
                let mut str = v.iter()
                    .map(|x| x.to_string())
                    .fold(String::new(), |acc, x| acc + &x + ",");
                str.pop();

                write!(f, "[{}]", str)
            },
            Expr::Bool(v) => v.fmt(f),
            Expr::Null => "null".fmt(f),
        }
    }
}
