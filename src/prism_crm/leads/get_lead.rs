use super::lead_types::Lead;
use crate::service::header_management::get_auth_headers;
use crate::types::Response;
use serde_json::Value;

use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
use serde_json::json;

pub async fn send_request(req: &HttpRequest, lead_id: &String) -> Response<Value> {
    let client = Client::default();
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(&req.headers());

    let response = client
        .get(format!("https://unify.apideck.com/crm/leads/{lead_id}"))
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send()
        .await;

    return response.unwrap().json::<Response<Value>>().await.unwrap();
}

pub async fn get_lead(req: HttpRequest, payload: web::Path<String>) -> HttpResponse {
    let lead_id = payload.into_inner();
    let response = send_request(&req, &lead_id).await;

    HttpResponse::Ok().json(json!(web::Json(response)))
}
