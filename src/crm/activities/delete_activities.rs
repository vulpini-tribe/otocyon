use super::_types::DeleteActivities;
use crate::service::req_client::req_client;

use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::json;

pub async fn delete_activities(req: HttpRequest, payload: Json<DeleteActivities>) -> HttpResponse {
    let client = req_client(&req);
    let futures = FuturesUnordered::new();

    payload.ids.iter().for_each(|activity_id| {
        let url = format!("https://unify.apideck.com/crm/activities/{activity_id}");
        let request = client.delete(url).send();

        futures.push(request);
    });

    futures.collect::<Vec<_>>().await;

    HttpResponse::Ok().json(json!(Json("Ok")))
}
