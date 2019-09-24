use clap::ArgMatches;

pub fn run<'a>(app: &ArgMatches<'a>) -> Result<(), &'static str> {
    Err("You must specify a command!")
}
