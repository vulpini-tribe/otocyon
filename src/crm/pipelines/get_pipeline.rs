use super::_types::Pipeline;
use crate::service::redix;
use crate::service::req_client::req_client;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::json;

pub async fn send_request(
    req: &HttpRequest,
    pipeline_id: &str,
    redis: web::Data<redis::Client>,
) -> Response<Pipeline> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/pipelines/{pipeline_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Pipeline>>().await;
    let pipeline = response.as_ref().unwrap().clone().data.unwrap();

    let futures = FuturesUnordered::new();

    pipeline.stages.clone().into_iter().for_each(|stage| {
        let key = format!("{}.{}", pipeline.id, stage.id);

        let request = redix::set_one(&redis, key, stage.name.to_string());
        futures.push(request);
    });

    futures.collect::<Vec<_>>().await;

    return response.unwrap();
}

pub async fn get_pipeline(
    req: HttpRequest,
    payload: web::Path<String>,
    redis: web::Data<redis::Client>,
) -> HttpResponse {
    let pipeline_id = payload.into_inner();
    let response = send_request(&req, &pipeline_id, redis).await;
    let pipeline = response.data.as_ref().expect("Pipeline is unknown");

    HttpResponse::Ok().json(json!(web::Json(pipeline)))
}
