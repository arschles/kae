use clap::{ArgMatches, App, SubCommand};

pub fn run<'a>(app: &ArgMatches<'a>) -> Result<(), &'static str> {
    print!("env was called!");
    return Ok(());
}

pub fn env_subcommand() -> App<'static, 'static> {
    let top_level = SubCommand::with_name("env")
        .about("Get and set environment variables");
    let set = SubCommand::with_name("set")
        .about("Sets an environment variable.");
    let get = SubCommand::with_name("get")
    .about("Get the value of an environment variable.");
    
    top_level.subcommand(set).subcommand(get)
}
