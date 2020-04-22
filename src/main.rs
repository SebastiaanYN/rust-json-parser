mod ast;
mod parse;
mod cursor;

fn main() {
    let str = &r#"
        [
            true,
            false,
            null,
            "Hello, World",
            "\"quote\" me on that",
            0,
            -42,
            5.25,
            {
                "a": [true, false, { "prop": null }],
                "b": {
                    "c": true,
                    "d": "\\\"\t"
                }
            }
        ]
    "#.to_string();

    match parse::parse(str) {
        Ok(v) => println!("{}", v),
        Err(v) => panic!(v),
    }
}
