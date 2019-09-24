use clap::{App, Arg};
use kube::api::Api;

pub mod kubernetes;
pub mod cmd;
use kubernetes::client::get_kube_client;
use cmd::environment::{env_subcommand, run_env};

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

    if matches.is_present("env") {
         match run_env(){
             Ok(()) => println!("done!"),
             Err(e) => println!("error! {}", e)
         };
    } else {
        let kube_client = get_kube_client(None);
        let deployments_resource = Api::v1Pod(kube_client)
            .within(DEFAULT_KUBE_NAMESPACE);
        match deployments_resource.get("service") {
            Ok(depl) => println!("got deployments {:?}", depl.spec.containers),
            Err(err) => println!("error getting deployments: {}", err),
        };
    }
}
