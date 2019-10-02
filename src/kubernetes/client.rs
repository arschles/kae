use crate::cmd::result::Res;
use http::request::Request;
use kube::{client::APIClient, config, Result as KubeResult};
use serde::de::DeserializeOwned;

// const DEFAULT_CONFIG_FILE: &str = "~/.kube/config";
pub const DEFAULT_NAMESPACE: &str = "default";
pub const DEFAULT_APP_NAME: &str = "kae";

fn load_kube_config() -> kube::Result<kube::config::Configuration> {
    // If env var is set, use in cluster config
    if std::env::var("KUBERNETES_PORT").is_ok() {
        return config::incluster_config();
    }
    config::load_kube_config()
}

pub fn get_kube_client() -> APIClient {
    let cfg = load_kube_config().expect("Couldn't load kube config");
    APIClient::new(cfg)
}

pub fn read_then_write<T>(
    cl: kube::client::APIClient,
    reader: fn() -> Request<Vec<u8>>,
    updater: fn(T) -> Result<T, String>,
) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let req = reader();
    let result: KubeResult<T> = cl.request(req);
    // TODO: check if it's completely missing and then just
    // create it if needed
    match result {
        Ok(res) => updater(res),
        Err(e) => Err(e.to_string()),
    }
}
