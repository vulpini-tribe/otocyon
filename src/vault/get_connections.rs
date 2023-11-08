use super::_types::{Connection, ConnectionFormattedList};
use crate::types::Response;

use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::{json, Value};

pub async fn get_connections(req: HttpRequest) -> HttpResponse {
    let response = req_client(&req)
        .get("https://unify.apideck.com/vault/connections")
        .send()
        .await
        .unwrap()
        .json::<Response<Vec<Value>>>()
        .await
        .unwrap();

    let connections: Vec<Value> = response
        .data
        .unwrap()
        .into_iter()
        // .map(|connection| connection.format_list())
        .map(|connection| connection)
        .collect();

    HttpResponse::Ok().json(json!(web::Json(connections)))
}
