use super::company_types::PostCompany;
use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::Value;

pub async fn send_request(req: &HttpRequest, payload: PostCompany) -> Value {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/companies");

    let response = client.post(url).send_json(&payload).await;
    let response = response.unwrap().json::<Value>().await;

    return response.unwrap();
}

pub async fn post_company(req: HttpRequest, payload: web::Json<PostCompany>) -> HttpResponse {
    let main_request = send_request(&req, payload.clone()).await;

    HttpResponse::Ok().json(web::Json(main_request))
}
