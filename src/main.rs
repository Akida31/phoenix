use std::io::Write;

//#[allow(dead_code)]
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
    let err = loop {
        let text = input!(">");
        match interpreter::run(text, "interpreter".to_string()) {
            Ok(_) => {}
            Err(e) => break e,
        };
    };
    println!("{}", err);
}
