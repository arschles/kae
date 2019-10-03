pub mod cmd;
pub mod kubernetes;
use cmd::{environment, result::err_res, result::Res};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Env {
    Get {
        #[structopt(short = "n", long = "name")]
        name: String,
    },
    Set {
        #[structopt(long = "name")]
        name: String,
        #[structopt(long = "val")]
        val: String,
    },
}

#[derive(StructOpt, Debug)]
pub enum SubCommands {
    Env(Env),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "kae", about = "Kubernetes App Engine")]
pub struct Opt {
    #[structopt(short = "k", long = "kubecfg")]
    kubeconfig: String,

    #[structopt(short = "v", long = "verbose")]
    verbose: String,
    #[structopt(subcommand)]
    cmds: SubCommands,
}

pub fn route(o: Opt) -> Res {
    match o.cmds {
        SubCommands::Env(Env::Get { name }) => environment::get(name),
        SubCommands::Env(Env::Set { name, val }) => environment::set(name, val),
        _ => err_res("Invalid command"),
    }
}
