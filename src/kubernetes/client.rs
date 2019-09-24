use kube::client::APIClient;

const DEFAULT_KUBE_CONFIG: &str = "~/.kube/config";

pub fn get_kube_client(opt_filename: Option<&str>) -> APIClient{
    let filename = opt_filename.unwrap_or(DEFAULT_KUBE_CONFIG);
    println!("Using kube config {}", filename);
    let cfg = kube::config::incluster_config()
        .or_else(|_| kube::config::load_kube_config())
        .expect("Failed to load kube config");
    APIClient::new(cfg)
}
