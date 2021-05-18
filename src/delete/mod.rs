use std::fs;
use std::path::PathBuf;

use crate::cli;
use plance::prompt;

pub fn delete_project(args: &cli::Cli) {
    if !prompt("Are you sure? You will lose your effort FOREVER! ") {
        return;
    }

    if !PathBuf::from(format!("{}/.plance", args.dir_name.as_str())).exists() {
        if !prompt("WARNING! The given directory is not a plance project. Proceed anyway?") {
            return;
        }
    }

    match fs::remove_dir_all(PathBuf::from(args.dir_name.clone())) {
        Ok(()) => (),
        Err(e) => {
            eprintln!(
                "ERROR! Couldn't remove directory {}: {}",
                args.dir_name.as_str(),
                e
            );
            return;
        }
    }
}
