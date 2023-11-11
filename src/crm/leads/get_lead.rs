use crate::service::req_client::req_client;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::{json, Value};

use super::_types::Lead;

pub async fn send_request(req: &HttpRequest, lead_id: &str) -> Response<Lead> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/leads/{lead_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Lead>>().await;

    return response.unwrap();
}

pub async fn get_lead(req: HttpRequest, payload: web::Path<String>) -> HttpResponse {
    let lead_id = payload.into_inner();
    let response = send_request(&req, &lead_id).await;

    HttpResponse::Ok().json(json!(web::Json(response)))
}
