use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::{json, Value};

pub async fn delete_company(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let client = req_client(&req);
    let company_id = path.into_inner();
    let url = format!("https://unify.apideck.com/crm/companies/{company_id}");

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
