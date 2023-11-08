use super::_types::PostPipeline;
use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::Value;

pub async fn post_pipeline(req: HttpRequest, payload: web::Json<PostPipeline>) -> HttpResponse {
    let response = req_client(&req)
        .post("https://unify.apideck.com/crm/pipelines")
        .send_json(&payload)
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    HttpResponse::Ok().json(web::Json(response))
}
