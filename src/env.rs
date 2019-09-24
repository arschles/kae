mod env;

use clap::SubCommand;

pub fn run_env() -> Result<(), Err> {
    print!("env was called!");
    return Ok(());
}

pub fn env_subcommand() -> SubCommand {
    SubCommand::with_name("env")
        .about("Get and set environment variables");
}
