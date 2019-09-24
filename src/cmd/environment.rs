use clap::{App, SubCommand};

pub fn run_env() -> Result<(), &'static str> {
    print!("env was called!");
    return Ok(());
}

pub fn env_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("env")
        .about("Get and set environment variables")
}
