use super::super::kubernetes::client;
use super::result::{err_res, Res};
use k8s_openapi::api::apps::v1::{DeploymentSpec, DeploymentStatus};
use k8s_openapi::api::core::v1::CreateNamespacedConfigMapOptional;
use k8s_openapi::api::core::v1::{ConfigMap, Secret};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, Object, PostParams};
use kube::Result as KubeResult;
use serde_json::{json, to_vec};
use std::collections::BTreeMap;

pub fn get<'a>(name: String) -> Res {
    let cl = client::get_kube_client();
    let config_map_res = Api::v1ConfigMap(cl)
        .within(client::DEFAULT_NAMESPACE)
        .get(&name);
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

pub fn set<'a>(name: String, val: String) -> Res {
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
