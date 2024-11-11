use clap::{command, Arg, Command};
//use std::path::PathBuf;

fn main() {
    let matches = command!()
        /*
        .arg(arg!([name] "Optional name"))
        .arg(
            arg!(-c --config <FILE> "Set custom config file")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(-d --debug ... "Turn on debugging"))
        */
        .subcommand(Command::new("list").about("List tasks"))
        .subcommand(
            Command::new("add")
                .about("Add a task")
                .arg(Arg::new("task")),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a task")
                .arg(Arg::new("task_id")),
        )
        .get_matches();
    println!("{:?}", matches);
    match matches.subcommand() {
        Some(("list", _)) => {
            println!("List items");
        }
        Some(("add", sub_matches)) => {
            println!("Add");
        }
        Some(("rm", _)) => {
            println!("Remove");
        }
        _ => {
            println!("ERROR")
        }
    }
}
