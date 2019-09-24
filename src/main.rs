use clap::{App, Arg};
// use kube::api::Api;

mod kubernetes;
mod cmd;
mod cli;
// use kubernetes::client::get_kube_client;
use cmd::environment::{env_subcommand};

const DEFAULT_KUBE_NAMESPACE: &str = "default";

fn main() {
    let matches = App::new("kae")
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
        .subcommand(env_subcommand())
        .get_matches();
        
    match cli::route(matches) {
        Ok(()) => {},
        Err(e) => println!("{}", e),
    };
    // match matches.subcommand() {
    //     Some("env") => run_env(matches.subcommand("env")),
    //     Some(name) => println!("Invalid command {}", name),
    //     None => println!("You need to specify a command!"),
    // }
    // else {
    //     let kube_client = get_kube_client(None);
    //     let deployments_resource = Api::v1Pod(kube_client)
    //         .within(DEFAULT_KUBE_NAMESPACE);
    //     match deployments_resource.get("service") {
    //         Ok(depl) => println!("got deployments {:?}", depl.spec.containers),
    //         Err(err) => println!("error getting deployments: {}", err),
    //     };
    // }
}
