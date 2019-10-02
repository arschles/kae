use super::cmd::deployment;
use super::cmd::{environment, result::err_res, result::Res};
use clap::{App, Arg, ArgMatches};

pub fn route(matches: ArgMatches) -> Res {
    let (sub, mb_sub_matches) = matches.subcommand();
    match mb_sub_matches {
        Some(sub_matches) => match sub {
            "env" => environment::route(&sub_matches),
            _ => err_res("Unknown command"),
        },
        None => err_res("Unknown command"),
    }
}

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("kae")
        .version("1.0")
        .author("Aaron S. <aaron@ecomaz.net>")
        .about("Kubernetes App Engine")
        .arg(
            Arg::with_name("kubeconfig")
                .short("k")
                .long("kubeconfig")
                .help("The Kubernetes configuration to use")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(environment::subcommand())
        .subcommand(deployment::subcommand())
}
