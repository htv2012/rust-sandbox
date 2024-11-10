use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;

fn main() {
    let matches = command!()
        .arg(arg!([name] "Optional name"))
        .arg(
            arg!(-c --config <FILE> "Set custom config file")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(-d --debug ... "Turn on debugging"))
        .subcommand(
            Command::new("test")
                .about("Perform test")
                .arg(arg!(-l --list "List test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    if let Some(name) = matches.get_one::<String>("name") {
        println!("name={}", name);
    }

    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("config_path={}", config_path.display());
    }
}
