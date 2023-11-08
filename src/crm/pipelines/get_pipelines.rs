use super::_types::{Pipeline, PipelineFormattedList};
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

    let pipelines: Vec<PipelineFormattedList> = response
        .data
        .unwrap()
        .into_iter()
        .map(|pipeline| pipeline.format_list())
        .collect();

    HttpResponse::Ok().json(json!(web::Json(pipelines)))
}
