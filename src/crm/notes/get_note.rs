use super::_types::Note;
use crate::service::req_client::req_client;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

pub async fn send_request(req: &HttpRequest, note_id: &str) -> Response<Note> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/notes/{note_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Note>>().await;

    return response.unwrap();
}

pub async fn get_note(
    req: HttpRequest,
    path: web::Path<String>,
    // redis: web::Data<redis::Client>,
) -> HttpResponse {
    let note_id = path.into_inner();

    let note = send_request(&req, &note_id).await;
    let note = note.data.as_ref().unwrap();

    let formatted = note.format_one();

    HttpResponse::Ok().json(json!(web::Json(formatted)))
}
