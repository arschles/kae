use clap::ArgMatches;
use k8s_openapi::api::core::v1::ConfigMap;

pub fn route<'a>(matches: &ArgMatches<'a>) -> Result<(), &'static str> {
    let (sub, mb_sub_matches) = matches.subcommand();
    match mb_sub_matches {
        Some(sub_matches) => match sub {
            "get" => get(sub_matches),
            "set" => set(sub_matches),
            _ => Err("Unknown command"),
        },
        _ => Err("Unknown command"),
    }
}

fn get<'a>(matches: &ArgMatches<'a>) -> Result<(), &'static str> {
    match matches.value_of("name") {
        Some(name) => {
            println!("Getting {}", name);
            Ok(())
        }
        _ => Err("Missing args!"),
    }
}

fn set<'a>(matches: &ArgMatches<'a>) -> Result<(), &'static str> {
    match (matches.value_of("name"), matches.value_of("value")) {
        (Some(name), Some(val)) => {
            println!("Setting {} = {}", name, val);
            Ok(())
        }
        _ => Err("Missing args!"),
    }
}
