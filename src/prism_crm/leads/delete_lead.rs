use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::{json, Value};

pub async fn delete_lead(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let lead_id = path.into_inner();
    let url = format!("https://unify.apideck.com/crm/contacts/{lead_id}");

    let response = req_client(&req)
        .delete(url)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    HttpResponse::Ok().json(json!(web::Json(response)))
}
