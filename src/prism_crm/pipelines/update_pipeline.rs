use super::_types::PostPipeline;
use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::Value;

pub async fn update_pipeline(req: HttpRequest, payload: web::Json<PostPipeline>) -> HttpResponse {
    let pipeline_id = req.match_info().get("pipeline_id").unwrap();
    let url = format!("https://unify.apideck.com/crm/pipelines/{pipeline_id}");

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
