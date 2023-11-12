use super::_types::PostNote;
use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::Value;

pub async fn update_note(req: HttpRequest, payload: web::Json<PostNote>) -> HttpResponse {
    let note_id = req.match_info().get("note_id").unwrap();
    let url = format!("https://unify.apideck.com/crm/notes/{note_id}");

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
