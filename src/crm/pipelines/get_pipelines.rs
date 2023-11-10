use super::_types::Pipeline;
use crate::service::req_client::req_client;
use crate::types::Response;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::service::redis::set_one;

pub async fn get_pipelines(req: HttpRequest, redis: web::Data<redis::Client>) -> HttpResponse {
    let query = [("limit", "20")];

    let _ = set_one(redis, "key-123".to_string(), "val-321".to_string()).await;

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
