use super::_types::DeleteNotes;
use crate::service::req_client::req_client;

use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::json;

pub async fn delete_notes(req: HttpRequest, payload: Json<DeleteNotes>) -> HttpResponse {
    let client = req_client(&req);
    let futures = FuturesUnordered::new();

    payload.ids.iter().for_each(|note_id| {
        let url = format!("https://unify.apideck.com/crm/notes/{note_id}");
        let request = client.delete(url).send();

        futures.push(request);
    });

    futures.collect::<Vec<_>>().await;

    HttpResponse::Ok().json(json!(Json("Ok")))
}
