use super::_types::PostContact;
use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::Value;

pub async fn update_contact(req: HttpRequest, payload: web::Json<PostContact>) -> HttpResponse {
    let contact_id = req.match_info().get("contact_id").unwrap();
    let url = format!("https://unify.apideck.com/crm/contacts/{contact_id}");

    let response = req_client(&req)
        .patch(url)
        .send_json(&payload)
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    HttpResponse::Ok().json(web::Json(response))
}
