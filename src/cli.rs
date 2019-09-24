use clap::{ArgMatches, App};
use super::cmd::environment::run_env;


pub fn route<'a>(app: ArgMatches<'a>) -> Result<(), &'a str> {
    let (cmd_name, subcommand_args) = match app.subcommand() {
        (cmd, Some(args)) => (cmd, args),
        _ => {
            return Err("Unknown command!");
        }
    };
    let cmd = match cmd_name {
        "env" => run_env,
        _ => return Err("Unknown command!"),
    };
    Ok(())
}
