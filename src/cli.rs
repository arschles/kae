use super::cmd::environment;
use clap::ArgMatches;

pub fn route<'a>(matches: ArgMatches<'a>) -> Result<(), &'static str> {
    let (sub, mb_sub_matches) = matches.subcommand();
    match mb_sub_matches {
        Some(sub_matches) => match sub {
            "env" => environment::route(&sub_matches),
            _ => Err("Unknown command"),
        },
        None => Err("Unknown command"),
    }
}
