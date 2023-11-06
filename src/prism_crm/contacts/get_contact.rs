use super::contact_types::Contact;
use crate::service::header_management::get_auth_headers;
use crate::types::Response;
use serde_json::Value;

use actix_web::{web, HttpRequest, HttpResponse};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;

pub async fn send_request(req: &HttpRequest, contact_id: &String) -> Response<Value> {
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(&req.headers());

    let client = reqwest::Client::new();
    let url = format!("https://unify.apideck.com/crm/contacts/{contact_id}");

    let response = client
        .get(url)
        .header(AUTHORIZATION, auth)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("x-apideck-app-id", app_id)
        .header("x-apideck-service-id", service_id)
        .header("x-apideck-consumer-id", consumer_id)
        .send()
        .await;

    let response = response.unwrap().json::<Response<Value>>().await;

    return response.unwrap();
}

pub async fn get_contact(req: HttpRequest, payload: web::Path<String>) -> HttpResponse {
    let contact_id = payload.into_inner();
    let response = send_request(&req, &contact_id).await;

    HttpResponse::Ok().json(json!(web::Json(response)))
}
