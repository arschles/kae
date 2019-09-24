mod kube;

use kube::client::APIClient

pub fn get_kube_client() -> APIClient{
    let kube_config = matches
        .value_of("kubeconfig")
        .unwrap_or(DEFAULT_KUBE_CONFIG);
    println!("Using kube config {}", kube_config);
    let cfg = kube::config::incluster_config()
        .or_else(|_| kube::config::load_kube_config())
        .expect("Failed to load kube config");
    APIClient::new(cfg);
}
