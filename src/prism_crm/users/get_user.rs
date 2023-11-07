use crate::service::req_client::req_client;
use crate::types::Response;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::{json, Value};

pub async fn send_request(req: &HttpRequest, user_id: &str) -> Response<Value> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/users/{user_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Value>>().await;

    return response.unwrap();
}

pub async fn get_user(req: HttpRequest, payload: web::Path<String>) -> HttpResponse {
    let user_id = payload.into_inner();
    let response = send_request(&req, &user_id).await;

    HttpResponse::Ok().json(json!(web::Json(response)))
}
