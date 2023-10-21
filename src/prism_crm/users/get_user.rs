use super::user_types::CrmUser;
use crate::service::header_management::get_auth_headers;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
use serde_json::json;

pub async fn send_request(req: &HttpRequest, user_id: &str) -> Response<CrmUser> {
    let client = Client::default();
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(&req.headers());

    let response = client
        .get(format!("https://unify.apideck.com/crm/users/{}", user_id))
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send()
        .await;

    return response.unwrap().json::<Response<CrmUser>>().await.unwrap();
}

pub async fn get_user(req: HttpRequest, payload: web::Path<String>) -> HttpResponse {
    let company_id = payload.into_inner();
    let response = send_request(&req, &company_id).await;

    HttpResponse::Ok().json(json!(web::Json(response)))
}
