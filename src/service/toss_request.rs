use super::req_client;
use actix_web::HttpRequest;
use serde_json::Value;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum RequestKinds {
    // COMPANY,
    LEAD,
    CONTACT,
    PIPELINE,
}

pub async fn toss_request(
    req: &HttpRequest,
    url: Arc<Mutex<String>>,
    kind: RequestKinds,
) -> (Value, RequestKinds) {
    let url = url.lock().unwrap().clone();
    let client = req_client::req_client(req);

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Value>().await.unwrap();

    (response, kind)
}
