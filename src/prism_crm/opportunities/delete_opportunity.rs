use crate::service::header_management::get_auth_headers;
use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
use serde_json::{json, Value};

pub async fn send_request(req: &HttpRequest, opportunity_id: &str) -> Value {
    let client = Client::default();
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(req.headers());

    let response = client
        .delete(format!(
            "https://unify.apideck.com/crm/opportunities/{opportunity_id}"
        ))
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send()
        .await;

    response.unwrap().json::<Value>().await.unwrap()
}

pub async fn delete_opportunity(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let opportunity_id = path.into_inner();
    let deleted = send_request(&req, &opportunity_id).await;

    HttpResponse::Ok().json(json!(web::Json(deleted)))
}
