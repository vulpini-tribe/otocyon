use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::{json, Value};

pub async fn delete_opportunity(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let client = req_client(&req);
    let opportunity_id = path.into_inner();
    let url = format!("https://unify.apideck.com/crm/opportunities/{opportunity_id}");

    let response = client
        .delete(url)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    HttpResponse::Ok().json(json!(web::Json(response)))
}
