use super::_types::Connection;
use crate::types::Response;

use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

pub async fn get_connections(req: HttpRequest) -> HttpResponse {
    let connections = req_client(&req)
        .get("https://unify.apideck.com/vault/connections")
        .send()
        .await
        .unwrap()
        .json::<Response<Vec<Connection>>>()
        .await
        .unwrap()
        .data
        .unwrap();

    HttpResponse::Ok().json(json!(web::Json(connections)))
}
