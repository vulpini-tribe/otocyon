// use super::pipeline_types::Pipeline;
use crate::service::header_management::get_auth_headers;
use crate::types::Response;
use actix_web::{web, HttpRequest, HttpResponse};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use serde_json::Value;

pub async fn send_request(req: &HttpRequest, pipeline_id: &String) -> Response<Value> {
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(&req.headers());

    let client = reqwest::Client::new();
    let url = format!("https://unify.apideck.com/crm/pipelines/{pipeline_id}");

    let response = client
        .get(url)
        .header(AUTHORIZATION, auth)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("x-apideck-app-id", app_id)
        .header("x-apideck-service-id", service_id)
        .header("x-apideck-consumer-id", consumer_id)
        .send()
        .await;

    let response = response.unwrap().json::<Response<Value>>().await;

    return response.unwrap();
}

pub async fn get_pipeline(req: HttpRequest, payload: web::Path<String>) -> HttpResponse {
    let pipeline_id = payload.into_inner();
    let response = send_request(&req, &pipeline_id).await;

    HttpResponse::Ok().json(json!(web::Json(response)))
}
