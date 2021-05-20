use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::cli;

pub mod files;

pub fn create_project(args: &cli::Cli) {
    // {name}
    //  ├── .plance
    //  │    └── info.json
    //  ├── src
    //  │    └── {main_file}
    //  └── README.md

    // First, create project structure:
    let source_dir = format!("{}/src", args.dir_name.as_str());
    let plance_dir = format!("{}/.plance", args.dir_name.as_str());

    match fs::create_dir_all(PathBuf::from(source_dir)) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("ERROR! Couldn't create source directory: {}", e);
            return;
        }
    }

    match fs::create_dir(PathBuf::from(plance_dir)) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("ERROR! Couldn't create Plance directory: {}", e);
            return;
        }
    }

    // Then, write to the files
    let mut file = match fs::File::create(PathBuf::from(format!(
        "{}/README.md",
        args.dir_name.as_str()
    ))) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("ERROR! Couldn't create README.md file: {}", e);
            return;
        }
    };
    match file.write_all(format!("# {}\n> *An incredible project*", args.name.as_str()).as_bytes())
    {
        Ok(()) => (),
        Err(e) => {
            eprintln!("ERROR! Couldn't write to README.md file: {}", e);
            return;
        }
    }

    let main = match args.project_type {
        cli::ProjectType::C => format!("{}/src/{}", args.dir_name.as_str(), files::C_FILE),
        cli::ProjectType::CPP => format!("{}/src/{}", args.dir_name.as_str(), files::CPP_FILE),
        cli::ProjectType::DEFAULT => {
            format!("{}/src/{}", args.dir_name.as_str(), files::DEFAULT_FILE)
        }
    };
    file = match fs::File::create(PathBuf::from(main)) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("ERROR! Couldn't create Main file: {}", e);
            return;
        }
    };

    match args.project_type {
        cli::ProjectType::C => match file.write_all(files::C_MAIN.as_bytes()) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("ERROR! Couldn't write to {} file: {}", files::C_FILE, e);
                return;
            }
        },
        cli::ProjectType::CPP => match file.write_all(files::CPP_MAIN.as_bytes()) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("ERROR! Couldn't write to {} file: {}", files::CPP_FILE, e);
                return;
            }
        },
        cli::ProjectType::DEFAULT => match file.write_all(files::DEFAULT_MAIN.as_bytes()) {
            Ok(()) => (),
            Err(e) => {
                eprintln!(
                    "ERROR! Couldn't write to {} file: {}",
                    files::DEFAULT_FILE,
                    e
                );
                return;
            }
        },
    }

    file = match fs::File::create(PathBuf::from(format!(
        "{}/.plance/info.json",
        args.dir_name.as_str()
    ))) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("ERROR! Couldn't create Plance Info file: {}", e);
            return;
        }
    };
    match file.write_all(format!("{{\n\t\"name\": \"{}\"\n}}", args.name.as_str()).as_bytes()) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("ERROR! Couldn't write to Plance Info file: {}", e);
            return;
        }
    }
}
