use crate::prism_crm::users::get_user;
use crate::service::header_management::get_headers;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Company {
    id: String,
    name: String,
    owner_id: String,
    owner: Option<String>,
}

pub async fn get_companies(req: HttpRequest) -> HttpResponse {
    let client = Client::default();
    let (app_id, auth, consumer_id, service_id) = get_headers(&req.headers());

    let response = client
        .get("https://unify.apideck.com/crm/companies") // <- Create request builder
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send() // <- Send http request
        .await;

    let res = response.unwrap().json::<Response<Company>>().await.unwrap();

    let _crm_user = get_user::send_request(&req, "512011392").await;

    println!("{:?}", _crm_user);

    // format received data to one unified format
    HttpResponse::Ok().json(json!(web::Json(res)))
}
