pub mod files;
pub mod info;

mod cli;
mod delete;
mod init;
mod new;

fn main() {
    // Get args from command line and execute the given subcommand
    let args = cli::Cli::get();
    match args.action {
        cli::Subcommand::New {
            name,
            dir_name,
            project_type,
            git_exists,
        } => new::create_project(name, dir_name, project_type, git_exists),
        cli::Subcommand::Init {
            name,
            project_type,
            create_src,
            git_exists,
        } => init::init_project(name, project_type, create_src, git_exists),
        cli::Subcommand::Delete { dir_name } => delete::delete_project(dir_name),
        cli::Subcommand::Default => (),
    };
}
