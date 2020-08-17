#![feature(try_trait)]

use std::io::Write;

mod interpreter;

macro_rules! input {
    () => {{
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Error reading input");
        buffer.trim().to_string()
    }};
    ($x:expr) => {{
        print!("{}", $x);
        std::io::stdout().flush().expect("Error writing output");
        input!()
    }};
}

fn main() {
    let mut stack = interpreter::new_stack();
    let version = env!("CARGO_PKG_VERSION");
    println!("Phoenix v{}", version);
    loop {
        let text = input!(">");
        let res = interpreter::run(text, "\"<stdin>\"".to_string(), Some(stack));
        match res.res {
            Ok(ty) if ty != interpreter::Type::none() => println!("{}", ty),
            Err(e) => println!("{}", e),
            _ => {} // don't show None Results
        };
        stack = res.stack;
    }
}
