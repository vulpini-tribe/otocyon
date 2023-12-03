use super::_types::{Note, NoteFormattedList};
use crate::types::Response;

use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

pub async fn get_notes(req: HttpRequest) -> HttpResponse {
    let query = [("limit", "100")];

    let response = req_client(&req)
        .get("https://unify.apideck.com/crm/notes")
        .query(&query)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json::<Response<Vec<Note>>>()
        .await
        .unwrap();

    let notes: Vec<NoteFormattedList> = response
        .data
        .unwrap_or(vec![])
        .into_iter()
        .map(|opportunity| opportunity.format_list())
        .collect();

    HttpResponse::Ok().json(json!(web::Json(notes)))
}
