use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use crate::cli;
use crate::files;
use crate::info;

pub fn create_project(
    name: String,
    dir_name: String,
    project_type: cli::ProjectType,
    git_exists: bool,
) {
    // {name}
    //  ├── .plance
    //  │    └── info.toml
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

    match fs::create_dir(PathBuf::from(plance_dir.to_owned())) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("ERROR! Couldn't create Plance directory: {}", e);
            return;
        }
    }

    // Then, write to the files
    let mut file = match File::create(PathBuf::from(format!("{}/README.md", dir_name))) {
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
    file = match File::create(PathBuf::from(main.to_owned())) {
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

    // Write the project information into .plance/info.toml
    let info = info::Info {
        project_info: info::ProjectInfo {
            name: name,
            project_type: match project_type {
                cli::ProjectType::C => String::from("c"),
                cli::ProjectType::Cpp => String::from("cpp"),
                cli::ProjectType::Default => String::from("default"),
            },
            files: Some(vec![main]),
        },
        build_opts: Some(info::BuildOpts {
            compiler: String::from("gcc"),

            // FIXME: try to implement an iterator over an array of &str to convert them to String
            flags: Some(vec!["-Wall".into(), "-O2".into()]),
        }),
    };
    let toml = toml::to_string(&info).expect("Cannot serialize given structure");

    file = match File::create(PathBuf::from(format!("{}/info.toml", plance_dir))) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("ERROR! Couldn't create Plance Info file: {}", e);
            return;
        }
    };
    match file.write_all(toml.as_bytes()) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("ERROR! Couldn't write to Plance Info file: {}", e);
            return;
        }
    }

    // Execute git in the current project
    if git_exists {
        Command::new("git")
            .arg("init")
            .arg("--quiet")
            .arg(dir_name)
            .spawn()
            .expect("failed to start git on the given directory");
    }
}
