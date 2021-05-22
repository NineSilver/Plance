use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use crate::cli;
use crate::files;

pub fn create_project(name: &str, dir_name: &str, project_type: cli::ProjectType, git_exists: bool) {
    // {name}
    //  ├── .plance
    //  │    └── info.json
    //  ├── src
    //  │    └── {main_file}
    //  └── README.md

    // First, create project structure:
    let source_dir = format!("{}/src", dir_name);
    let plance_dir = format!("{}/.plance", dir_name);

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
    let mut file = match fs::File::create(PathBuf::from(format!("{}/README.md", dir_name)))
    {
        Ok(result) => result,
        Err(e) => {
            eprintln!("ERROR! Couldn't create README.md file: {}", e);
            return;
        }
    };
    match file.write_all(format!("# {}\n> *An incredible project*", name).as_bytes()) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("ERROR! Couldn't write to README.md file: {}", e);
            return;
        }
    }

    let main = match project_type {
        cli::ProjectType::C => format!("{}/src/{}", dir_name, files::C_FILE),
        cli::ProjectType::Cpp => format!("{}/src/{}", dir_name, files::CPP_FILE),
        cli::ProjectType::Default => {
            format!("{}/src/{}", dir_name, files::DEFAULT_FILE)
        }
    };
    file = match fs::File::create(PathBuf::from(main)) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("ERROR! Couldn't create Main file: {}", e);
            return;
        }
    };

    match project_type {
        cli::ProjectType::C => match file.write_all(files::C_MAIN.as_bytes()) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("ERROR! Couldn't write to {} file: {}", files::C_FILE, e);
                return;
            }
        },
        cli::ProjectType::Cpp => match file.write_all(files::CPP_MAIN.as_bytes()) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("ERROR! Couldn't write to {} file: {}", files::CPP_FILE, e);
                return;
            }
        },
        cli::ProjectType::Default => match file.write_all(files::DEFAULT_MAIN.as_bytes()) {
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
        dir_name
    ))) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("ERROR! Couldn't create Plance Info file: {}", e);
            return;
        }
    };
    match file.write_all(
        format!(
            "{{\n\t\"name\": \"{}\",\n\t\"type\": \"{}\"\n}}",
            name,
            match project_type {
                cli::ProjectType::C => "c",
                cli::ProjectType::Cpp => "cpp",
                cli::ProjectType::Default => "default",
            }
        )
        .as_bytes(),
    ) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("ERROR! Couldn't write to Plance Info file: {}", e);
            return;
        }
    }

    if git_exists {
        Command::new("git").arg("init").arg("--quiet").arg(dir_name).spawn().expect("failed to start git on the given directory");
    }
}
