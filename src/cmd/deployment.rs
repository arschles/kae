use clap::{App, Arg, SubCommand};
use k8s_openapi::api::apps::v1::{DeploymentSpec, DeploymentStatus};
use kube::api::Object;

type Deployment = Object<DeploymentSpec, DeploymentStatus>;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("deploy")
        .about("Deploy your application")
        .arg(
            Arg::with_name("image")
                .long("image")
                .required(true)
                .index(1)
                .takes_value(true),
        )
}
