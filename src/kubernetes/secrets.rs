use kube::client::APIClient;
use kube::api::Api;
// use k8s_openapi::api::extensions::v1beta1::deployment;
use k8s_openapi::api::core::v1::SecretList;

pub fn list_secrets(client: APIClient) -> SecretList {
    unimplemented!()
}
// et kube_client = get_kube_client(None);
    //     let deployments_resource = Api::v1Pod(kube_client)
    //         .within(DEFAULT_KUBE_NAMESPACE);
    //     match deployments_resource.get("service") {
    //         Ok(depl) => println!("got deployments {:?}", depl.spec.containers),
    //         Err(err) => println!("error getting deployments: {}", err),
    //     };
