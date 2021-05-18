use clap::{App, AppSettings, Arg, SubCommand};

pub enum Subcommand {
    NEW,
    // INIT,
    DELETE,
    DEFAULT,
}

pub enum ProjectType {
    C,
    CPP,
    DEFAULT,
}

pub struct Cli {
    pub action: Subcommand,

    pub name: String,
    pub dir_name: String,
    pub project_type: ProjectType,
}

impl Cli {
    fn new() -> Self {
        Cli {
            action: Subcommand::DEFAULT,

            name: String::new(),
            dir_name: String::new(),
            project_type: ProjectType::DEFAULT,
        }
    }

    pub fn get() -> Self {
        let matches = App::new("Plance")
            .version("0.1.0")
            .author("NineSilver")
            .about("Manage C/C++ projects the way Cargo does")
            .subcommand(
                SubCommand::with_name("new")
                    .about("Create a new and empty project")
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
                SubCommand::with_name("delete")
                    .about("Delete an existing project")
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

        let mut cli = Self::new();

        if let Some(matches) = matches.subcommand_matches("new") {
            cli.action = Subcommand::NEW;
            cli.name.push_str(matches.value_of("name").unwrap());

            if matches.is_present("dirname") {
                cli.dir_name.push_str(matches.value_of("dirname").unwrap());
            } else {
                cli.dir_name = cli.name.clone();
            }

            if matches.is_present("type") {
                match matches.value_of("type") {
                    Some("c") | Some("C") => cli.project_type = ProjectType::C,
                    Some("cpp") | Some("CPP") | Some("c++") | Some("C++") => {
                        cli.project_type = ProjectType::CPP
                    }
                    Some("default") | Some("DEFAULT") | Some(_) | None => {
                        cli.project_type = ProjectType::DEFAULT
                    }
                }
            } else {
                cli.project_type = ProjectType::DEFAULT;
                eprintln!("WARNING! No project type supplied, defaulting...");
            }
        } else if let Some(matches) = matches.subcommand_matches("delete") {
            cli.action = Subcommand::DELETE;
            cli.dir_name.push_str(matches.value_of("dirname").unwrap());
        }

        cli
    }
}
