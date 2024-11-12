use clap::{arg, command, Command};
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
        .subcommand(Command::new("add").about("Add a task").arg(arg!([TASK])))
        .subcommand(
            Command::new("rm")
                .about("Remove a task")
                .arg(arg!([TASK_ID])),
        )
        .get_matches();
    println!("{:?}", matches);
    match matches.subcommand() {
        Some(("list", _)) => {
            println!("List items");
        }
        Some(("add", sub_matches)) => {
            let task = sub_matches.get_one::<String>("TASK").unwrap();
            println!("Add >{}<", task);
        }
        Some(("rm", _)) => {
            println!("Remove");
        }
        _ => {
            println!("ERROR")
        }
    }
}
