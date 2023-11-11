use super::_types::Pipeline;
use crate::service::req_client::req_client;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::{json, Value};

pub async fn send_request(req: &HttpRequest, pipeline_id: &str) -> Response<Pipeline> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/pipelines/{pipeline_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Pipeline>>().await;

    return response.unwrap();
}

pub async fn get_pipeline(req: HttpRequest, payload: web::Path<String>) -> HttpResponse {
    let pipeline_id = payload.into_inner();
    let response = send_request(&req, &pipeline_id).await;

    HttpResponse::Ok().json(json!(web::Json(response)))
}
