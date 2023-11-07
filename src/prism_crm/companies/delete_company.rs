use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::{json, Value};

pub async fn send_request(req: &HttpRequest, company_id: String) -> Value {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/companies/{company_id}");

    let response = client.delete(url).send().await;
    let response = response.unwrap().json::<Value>().await;

    return response.unwrap();
}

pub async fn delete_company(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let company_id = path.into_inner();
    let deleted = send_request(&req, company_id).await;

    HttpResponse::Ok().json(json!(web::Json(deleted)))
}
