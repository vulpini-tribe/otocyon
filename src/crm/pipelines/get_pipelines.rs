use super::_types::Pipeline;
use crate::service::redix;
use crate::service::req_client::req_client;
use crate::types::Response;
use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::json;

pub async fn get_pipelines(req: HttpRequest, redis: web::Data<redis::Client>) -> HttpResponse {
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
    let futures = FuturesUnordered::new();

    pipelines.iter().for_each(|pipeline| {
        pipeline.stages.clone().into_iter().for_each(|stage| {
            let key = format!("{}.{}", pipeline.id, stage.id);

            let request = redix::set_one(&redis, key, stage.name.to_string());
            futures.push(request);
        });
    });

    futures.collect::<Vec<_>>().await;

    HttpResponse::Ok().json(json!(web::Json(pipelines)))
}
