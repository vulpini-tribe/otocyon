use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::{json, Value};

pub async fn send_request(req: &HttpRequest, opportunity_id: &str) -> Value {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/opportunities/{opportunity_id}");

    let response = client.delete(url).send().await;
    let response = response.unwrap().json::<Value>().await;

    return response.unwrap();
}

pub async fn delete_opportunity(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let opportunity_id = path.into_inner();
    let deleted = send_request(&req, &opportunity_id).await;

    HttpResponse::Ok().json(json!(web::Json(deleted)))
}
