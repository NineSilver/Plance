use clap::{App, AppSettings, Arg, SubCommand};

use plance::is_program_in_path;

pub enum Subcommand {
    New {
        name: String,
        dir_name: String,
        project_type: ProjectType,
        git_exists: bool
    },
    Init {
        name: Option<String>,
        project_type: ProjectType,
        create_src: bool,
        git_exists: bool
    },
    Delete {
        dir_name: String,
    },
    Default,
}

pub enum ProjectType {
    C,
    Cpp,
    Default,
}

pub struct Cli {
    pub action: Subcommand,
}

impl Cli {
    // Create a brand-new Cli struct
    fn new() -> Self {
        Cli {
            action: Subcommand::Default,
        }
    }

    // Get args from command line
    pub fn get() -> Self {
        // Use clap to parse the given arguments
        let matches = App::new("Plance")
            .version("0.1.0")
            .author("NineSilver")
            .about("Manage C/C++ projects the way Cargo does")
            .subcommand(
                SubCommand::with_name("new")
                    .about("Create a new and empty project")
                    .alias("create")
                    .arg(
                        Arg::with_name("name")
                            .help("Name of the project you wanna create")
                            .index(1)
                            .required(true)
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name("dirname")
                            .help("Name of the directory which will contain the project")
                            .short("C")
                            .long("dir")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name("type")
                            .help("Project type")
                            .short("t")
                            .long("type")
                            .alias("kind")
                            .takes_value(true)
                            .possible_values(&[
                                "default", "DEFAULT", "c", "C", "cpp", "CPP", "c++", "C++",
                            ]),
                    ),
            )
            .subcommand(
                SubCommand::with_name("init")
                    .about("Initialize a Plance project in the existing directory")
                    .arg(
                        Arg::with_name("name")
                            .help("Specify the name of the project. Defaults to current working directory")
                            .long("name")
                            .required(false)
                            .takes_value(true)
                    )
                    .arg(
                        Arg::with_name("type")
                            .help("Project type")
                            .short("t")
                            .long("type")
                            .alias("kind")
                            .takes_value(true)
                            .possible_values(&[
                                "default", "DEFAULT", "c", "C", "cpp", "CPP", "c++", "C++",
                            ]),
                    )
                    .arg(
                        Arg::with_name("createsource")
                            .help("Create a source directory")
                            .long("create-source")
                            .takes_value(false)
                    )
            )
            .subcommand(
                SubCommand::with_name("delete")
                    .about("Delete an existing project")
                    .aliases(&["remove", "erase"])
                    .arg(
                        Arg::with_name("dirname")
                            .help("Directory name of the project to be removed")
                            .index(1)
                            .required(true)
                            .takes_value(true),
                    ),
            )
            .setting(AppSettings::ArgRequiredElseHelp)
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .get_matches();

        // Create a new struct
        let mut cli = Self::new();

        // Program logic
        if let Some(matches) = matches.subcommand_matches("new") {
            cli.action = Subcommand::New {
                name: String::from(matches.value_of("name").unwrap()),
                dir_name: match matches.value_of("dirname") {
                    Some(r) => String::from(r),
                    None => String::from(matches.value_of("name").unwrap()),
                },
                project_type: match matches.value_of("type") {
                    Some("c") | Some("C") => ProjectType::C,
                    Some("cpp") | Some("CPP") | Some("c++") | Some("C++") => ProjectType::Cpp,
                    Some("default") | Some("DEFAULT") | Some(_) | None => ProjectType::Default,
                },
                git_exists: is_program_in_path("git")
            };
        } else if let Some(matches) = matches.subcommand_matches("init") {
            cli.action = Subcommand::Init {
                name: match matches.value_of("name") {
                    Some(v) => Some(String::from(v)),
                    None => None,
                },
                project_type: match matches.value_of("type") {
                    Some("c") | Some("C") => ProjectType::C,
                    Some("cpp") | Some("CPP") | Some("c++") | Some("C++") => ProjectType::Cpp,
                    Some("default") | Some("DEFAULT") | Some(_) | None => ProjectType::Default,
                },
                create_src: matches.is_present("createsource"),
                git_exists: is_program_in_path("git")
            }
        } else if let Some(matches) = matches.subcommand_matches("delete") {
            cli.action = Subcommand::Delete {
                dir_name: String::from(matches.value_of("dirname").unwrap()),
            }
        }

        // Return the processed struct
        cli
    }
}
