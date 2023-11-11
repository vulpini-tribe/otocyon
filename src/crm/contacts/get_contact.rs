use crate::service::req_client::req_client;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::{json, Value};

use super::_types::Contact;

pub async fn send_request(req: &HttpRequest, contact_id: &str) -> Response<Contact> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/contacts/{contact_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Contact>>().await;

    return response.unwrap();
}

pub async fn get_contact(req: HttpRequest, payload: web::Path<String>) -> HttpResponse {
    let contact_id = payload.into_inner();
    let response = send_request(&req, &contact_id).await;

    HttpResponse::Ok().json(json!(web::Json(response)))
}
