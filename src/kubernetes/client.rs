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
    creater: fn() -> Request<Vec<u8>>,
    updater: fn(T) -> Result<T, String>,
) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let read_req = reader();
    let read_res: KubeResult<T> = cl.request(read_req);
    let ret: Result<T, String> = match read_res {
        Ok(res) => {
            let res: Result<T, String> = updater(res);
            res
        }
        // TODO: probably better to check for a specific error message
        // indicating the thing wasn't there...
        Err(_) => {
            let req = creater();
            match cl.request(req) {
                Ok(t) => {
                    let updated: Result<T, String> = updater(t);
                    updated
                }
                Err(e) => {
                    let err: Result<T, String> = Err(e.to_string());
                    err
                }
            }
        }
    };
    ret
}
