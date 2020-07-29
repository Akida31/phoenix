mod basic;

use std::io::Write;

macro_rules! input {
    () => {{
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Error reading input");
        buffer.trim().to_owned()
    }};
    ($x:expr) => {{
        print!("{}", $x);
        std::io::stdout().flush().expect("Error writing output");
        input!()
    }};
}

fn main() {
    let err = loop {
        let text = input!(">");
        match basic::run(text, "interpreter".to_owned()) {
            Ok(text) => println!("{:?}", text),
            Err(e) => break e,
        };
    };
    println!("{}", err);
}
