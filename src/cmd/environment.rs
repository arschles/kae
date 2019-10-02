use super::super::kubernetes::client;
use super::result::{err_res, Res};
use clap::{App, Arg, ArgMatches, SubCommand};
use k8s_openapi::api::apps::v1::{DeploymentSpec, DeploymentStatus};
use k8s_openapi::api::core::v1::CreateNamespacedConfigMapOptional;
use k8s_openapi::api::core::v1::{ConfigMap, Secret};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, Object, PostParams};
use kube::Result as KubeResult;
use serde_json::{json, to_vec};
use std::collections::BTreeMap;

pub fn route<'a>(matches: &ArgMatches<'a>) -> Res {
    let (sub, mb_sub_matches) = matches.subcommand();
    match mb_sub_matches {
        Some(sub_matches) => match sub {
            "get" => get(sub_matches),
            "set" => set(sub_matches),
            _ => Err(String::from("Unknown command")),
        },
        _ => err_res("Unknown command"),
    }
}

fn get<'a>(matches: &ArgMatches<'a>) -> Res {
    match matches.value_of("name") {
        Some(name) => {
            let cl = client::get_kube_client();
            let config_map_res = Api::v1ConfigMap(cl)
                .within(client::DEFAULT_NAMESPACE)
                .get(name);
            match config_map_res {
                Ok(cm) => {
                    println!("{}", cm.metadata.name);
                    Ok(())
                }
                _ => {
                    // let fmt_str = format!("Couldn't find config map {}", name).to_owned();
                    // let ret_str: &str
                    err_res("Couldn't find config map")
                }
            }
        }
        _ => err_res("Missing args!"),
    }
}

fn set<'a>(matches: &ArgMatches<'a>) -> Res {
    match (matches.value_of("name"), matches.value_of("value")) {
        (Some(name), Some(val)) => {
            println!("{} = {}", name, val);
            let cl = client::get_kube_client();
            let mut vals: BTreeMap<String, String> = BTreeMap::new();
            vals.insert(String::from(name), String::from(val));
            // let config_map = Api::v1ConfigMap(cl).within(client::DEFAULT_NAMESPACE);
            let config_map = ConfigMap {
                binary_data: None,
                data: Some(vals),
                metadata: Some(ObjectMeta {
                    annotations: None,
                    cluster_name: None,
                    creation_timestamp: None,
                    deletion_grace_period_seconds: None,
                    deletion_timestamp: None,
                    finalizers: None,
                    generate_name: None,
                    generation: None,
                    initializers: None,
                    labels: None,
                    managed_fields: None,
                    name: Some(String::from(client::DEFAULT_APP_NAME)),
                    namespace: Some(String::from(client::DEFAULT_NAMESPACE)),
                    owner_references: None,
                    resource_version: None,
                    self_link: None,
                    uid: None,
                }),
            };
            let create_res = ConfigMap::create_namespaced_config_map(
                client::DEFAULT_NAMESPACE,
                &config_map,
                CreateNamespacedConfigMapOptional::default(),
            );
            match create_res {
                Ok((req, _)) => {
                    let cm_res: KubeResult<ConfigMap> = cl.request(req);
                    cm_res.map(|_| ()).map_err(|e| e.to_string())
                }
                Err(e) => Err(e.to_string()),
            }
        }
        _ => err_res("Missing args!"),
    }
}

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("env")
        .subcommand(
            SubCommand::with_name("get")
                .about("Get an environment variable")
                .arg(
                    Arg::with_name("name")
                        .long("name")
                        .required(true)
                        .index(1)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set an environment variable")
                .arg(
                    Arg::with_name("name")
                        .index(1)
                        .short("name")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("value")
                        .index(2)
                        .short("value")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .about("Interact with environment variables")
}
