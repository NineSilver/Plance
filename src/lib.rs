use std::io;
use std::io::Write;

pub fn prompt(msg: &str) -> bool {
    print!("{} [y/N] ", msg);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim_end().to_lowercase() == "y",
        Err(e) => {
            eprintln!("ERROR! Couldn't read from stdin: {}", e);
            false
        }
    }
}
