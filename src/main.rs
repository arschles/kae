use clap::{App, Arg, SubCommand};
// use kube::api::Api;

mod cli;
mod cmd;
mod kubernetes;
// use kubernetes::client::get_kube_client;

fn main() {
    let app = App::new("kae")
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
        .subcommand(
            SubCommand::with_name("env")
                .subcommand(
                    SubCommand::with_name("get")
                        .about("Gets an environment variable")
                        .arg(Arg::with_name("name").index(1))
                        .arg(Arg::with_name("value").index(2)),
                )
                .subcommand(
                    SubCommand::with_name("set")
                        .about("Sets and environment variable")
                        .arg(Arg::with_name("name").index(1))
                        .arg(Arg::with_name("value").index(2)),
                )
                .about("Interacts with environment variables"),
        );
    let matches = app.clone().get_matches();
    match cli::route(matches) {
        Ok(()) => {}
        Err(s) => {
            print!("{}!\n\n", s);
            app.clone().print_long_help();
        }
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
