use std::fs;
use std::io::Write;
use std::option::Option;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::cli;
use crate::files;

pub fn init_project(name: Option<String>, project_type: cli::ProjectType, create_source: bool, git_exists: bool) {
    let cwd = std::env::current_dir().unwrap();
    let cwd_name = cwd.file_name().unwrap().to_str().unwrap();
    let cwd = cwd.as_path();

    let plance_dir = format!("{}/.plance", cwd.to_str().unwrap());
    if Path::exists(Path::new(&plance_dir)) {
        eprintln!("ERROR! It seems like the current directory already has a Plance project in it");
        return;
    }

    let source = format!("{}/src", cwd.to_str().unwrap());

    match fs::create_dir(Path::join(cwd, PathBuf::from(plance_dir.clone()))) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("ERROR! Couldn't create Plance directory: {}", e);
            return;
        }
    };

    if create_source {
        match fs::create_dir(Path::join(cwd, PathBuf::from(source.clone()))) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("ERROR! Couldn't create source directory: {}", e);
                return;
            }
        };

        match project_type {
            cli::ProjectType::C => {
                let mut file = match fs::File::create(PathBuf::from(format!(
                    "{}/{}",
                    source.as_str(),
                    files::C_FILE
                ))) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("ERROR! Couldn't create Main file: {}", e);
                        return;
                    }
                };
                match file.write_all(files::C_MAIN.as_bytes()) {
                    Ok(()) => (),
                    Err(e) => {
                        eprintln!("ERROR! Couldn't write to Main file: {}", e);
                        return;
                    }
                };
            }
            cli::ProjectType::Cpp => {
                let mut file = match fs::File::create(PathBuf::from(format!(
                    "{}/{}",
                    source.as_str(),
                    files::CPP_FILE
                ))) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("ERROR! Couldn't create Main file: {}", e);
                        return;
                    }
                };
                match file.write_all(files::CPP_MAIN.as_bytes()) {
                    Ok(()) => (),
                    Err(e) => {
                        eprintln!("ERROR! Couldn't write to Main file: {}", e);
                        return;
                    }
                };
            }
            cli::ProjectType::Default => {
                let mut file = match fs::File::create(PathBuf::from(format!(
                    "{}/{}",
                    source.as_str(),
                    files::DEFAULT_FILE
                ))) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("ERROR! Couldn't create Main file: {}", e);
                        return;
                    }
                };
                match file.write_all(files::DEFAULT_MAIN.as_bytes()) {
                    Ok(()) => (),
                    Err(e) => {
                        eprintln!("ERROR! Couldn't write to Main file: {}", e);
                        return;
                    }
                };
            }
        }
    }

    let mut file =
        match fs::File::create(PathBuf::from(format!("{}/info.json", plance_dir.as_str()))) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("ERROR! Couldn't create Plance Info file: {}", e);
                return;
            }
        };
    match file.write_all(
        format!(
            "{{\n\t\"name\": \"{}\",\n\t\"type\": \"{}\"\n}}",
            match &name {
                Some(s) => s,
                None => cwd_name,
            },
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
        Command::new("git").arg("init").arg("--quiet").arg(cwd.to_str().unwrap()).spawn().expect("failed to start git on the given directory");
    }
}
