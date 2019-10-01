use clap::{App, ArgMatches, SubCommand};
use k8s_openapi::api::core::v1::ConfigMap;

pub fn run<'a>(app: &ArgMatches<'a>) -> Result<(), &'static str> {
    let name = app.value_of("name");
    let val = app.value_of("val");
    print!(
        "env was called! name = {}, val = {}",
        name.unwrap(),
        val.unwrap()
    );
    // let cm = ConfigMap::create_namespaced_config_map(
    //     DEFAULT_KUBE_NAMESPACE,
    //     )
    Ok(())
}

pub fn env_subcommand() -> App<'static, 'static> {
    let top_level = SubCommand::with_name("env").about("Get and set environment variables");
    let set = SubCommand::with_name("set").about("Sets an environment variable.");
    let get = SubCommand::with_name("get").about("Get the value of an environment variable.");
    top_level.subcommand(set).subcommand(get)
}
