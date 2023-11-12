use super::_types::PostNote;
use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::Value;

pub async fn post_note(req: HttpRequest, payload: web::Json<PostNote>) -> HttpResponse {
    let response = req_client(&req)
        .post("https://unify.apideck.com/crm/notes")
        .send_json(&payload)
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    HttpResponse::Ok().json(web::Json(response))
}
