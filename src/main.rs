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
    loop {
        let text = input!(">");
        match interpreter::run(text, "\"<stdin>\"".to_string()) {
            Ok(ty) => println!("{}", ty),
            Err(e) => println!("{}", e),
        };
    }
}
