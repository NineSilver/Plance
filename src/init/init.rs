use std::fs;
use std::fs::File;
use std::io::Write;
use std::option::Option;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::cli;
use crate::files;
use crate::info;

pub fn init_project(
    name: Option<String>,
    project_type: cli::ProjectType,
    create_source: bool,
    git_exists: bool,
) {
    let cwd = std::env::current_dir().unwrap();
    let cwd_name = cwd.file_name().unwrap().to_str().unwrap();
    let cwd = cwd.as_path();

    let plance_dir = format!("{}/.plance", cwd.to_str().unwrap());
    if Path::exists(Path::new(&plance_dir)) {
        eprintln!("ERROR! It seems like the current directory already has a Plance project in it");
        return;
    }

    let source = format!("{}/src", cwd.to_str().unwrap());

    match fs::create_dir(Path::join(cwd, PathBuf::from(plance_dir.to_owned()))) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("ERROR! Couldn't create Plance directory: {}", e);
            return;
        }
    };

    // Get the info using toml-rs
    let mut info = info::Info {
        project_info: info::ProjectInfo {
            name: match name {
                Some(s) => s,
                None => String::from(cwd_name),
            },
            project_type: match project_type {
                cli::ProjectType::C => String::from("c"),
                cli::ProjectType::Cpp => String::from("cpp"),
                cli::ProjectType::Default => String::from("default"),
            },
            files: None,
        },
        build_opts: Some(info::BuildOpts {
            compiler: String::from("gcc"),

            // FIXME: try to implement an iterator over an array of &str to convert them to String
            flags: Some(vec!["-Wall".into(), "-O2".into()]),
        }),
    };

    // Create source directory (if required)
    if create_source {
        match fs::create_dir(Path::join(cwd, PathBuf::from(source.to_owned()))) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("ERROR! Couldn't create source directory: {}", e);
                return;
            }
        };

        match project_type {
            cli::ProjectType::C => {
                let main = format!("{}/{}", source.as_str(), files::C_FILE);

                let mut file = match File::create(PathBuf::from(main.to_owned())) {
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

                info.project_info.files = Some(vec![main]);
            }
            cli::ProjectType::Cpp => {
                let main = format!("{}/{}", source.as_str(), files::CPP_FILE);

                let mut file = match File::create(PathBuf::from(main.to_owned())) {
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

                info.project_info.files = Some(vec![main]);
            }
            cli::ProjectType::Default => {
                let main = format!("{}/{}", source.as_str(), files::DEFAULT_FILE);

                let mut file = match File::create(PathBuf::from(main.to_owned())) {
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

                info.project_info.files = Some(vec![main]);
            }
        }
    }

    // Write the TOML formatted struct to the file
    let toml = toml::to_string(&info).expect("Cannot serialize given structure");

    let mut file = match File::create(PathBuf::from(format!("{}/info.toml", plance_dir))) {
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

    // Run git
    if git_exists {
        Command::new("git")
            .arg("init")
            .arg("--quiet")
            .arg(cwd.to_str().unwrap())
            .spawn()
            .expect("failed to start git on the given directory");
    }
}
