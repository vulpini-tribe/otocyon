use super::_types::Pipeline;
use crate::types::Response;

use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

pub async fn get_pipelines(req: HttpRequest) -> HttpResponse {
    let query = [("limit", "20")];

    let response = req_client(&req)
        .get("https://unify.apideck.com/crm/pipelines")
        .query(&query)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json::<Response<Vec<Pipeline>>>()
        .await
        .unwrap();

    let pipelines: Vec<Pipeline> = response.data.unwrap();

    HttpResponse::Ok().json(json!(web::Json(pipelines)))
}
