use super::super::kubernetes::client;
use clap::ArgMatches;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::api::{Api, PostParams};
use serde_json::{json, to_vec};
use std::collections::BTreeMap;

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
            let cl = client::get_kube_client();
            let config_map_res = Api::v1ConfigMap(cl)
                .within(client::DEFAULT_NAMESPACE)
                .get(name);
            match config_map_res {
                Ok(cm) => {
                    println!("{}", cm.metadata.name);
                    Ok(())
                }
                _ => Err("Couldn't find config map"),
            }
        }
        _ => Err("Missing args!"),
    }
}

fn set<'a>(matches: &ArgMatches<'a>) -> Result<(), &'static str> {
    match (matches.value_of("name"), matches.value_of("value")) {
        (Some(name), Some(val)) => {
            let cl = client::get_kube_client();
            let mut configMap = Api::v1ConfigMap(cl).within(client::DEFAULT_NAMESPACE);
            let mut vals = BTreeMap::new();
            vals.insert(name, val);
            let json = json!({
                name: val,
            });
            let res_vec = to_vec(&json);
            match res_vec {
                Ok(vec) => {
                    configMap.create(&PostParams::default(), vec);
                    Ok(())
                }
                _ => Err("Couldn't create Kubernetes configmap"),
            }
        }
        _ => Err("Missing args!"),
    }
}
