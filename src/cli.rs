use clap::ArgMatches;
use super::cmd::{environment, empty};


pub fn route<'a>(app: ArgMatches<'a>) -> Result<(), &'static str> {
    let (cmd_name, subcommand_args) = match app.subcommand() {
        (cmd, Some(args)) => (cmd, args),
        _ => {
            return Err("Unknown command!");
        }
    };
    println!("command name: {}", cmd_name);
    let action = match cmd_name {
        "env" => environment::run,
        _ => empty::run,
    };
    action(subcommand_args)
}
