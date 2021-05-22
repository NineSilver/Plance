use std::env;
use std::fs;
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

pub fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        if cfg!(target_os = "windows") {
            for p in path.split(';') {
                let p_str = format!("{}/{}", p, program);
                if fs::metadata(p_str).is_ok() {
                    return true;
                }
            }
        } else {
            for p in path.split(':') {
                let p_str = format!("{}/{}", p, program);
                if fs::metadata(p_str).is_ok() {
                    return true;
                }
            }
        }
    }
    false
}
