use crate::cmd::result::Res;
use http::request::Request;
use kube::{client::APIClient, config};

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
    reader: fn(&str, &str, T) -> Request<Vec<u8>>,
    writer: fn(&str, &str, &T) -> Request<Vec<u8>>,
) -> Res {
    Ok(())
}
