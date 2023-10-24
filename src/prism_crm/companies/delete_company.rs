use crate::service::header_management::get_auth_headers;
use crate::types::{DeletePayload, Response};
use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
use serde_json::json;

pub async fn send_request(req: HttpRequest, company_id: String) -> Response<DeletePayload> {
    let client = Client::default();
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(&req.headers());

    let response = client
        .delete(format!(
            "https://unify.apideck.com/crm/companies/{company_id}"
        ))
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send()
        .await;

    response
        .unwrap()
        .json::<Response<DeletePayload>>()
        .await
        .unwrap()
}

pub async fn delete_company(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let company_id = path.into_inner();
    let deleted = send_request(req, company_id).await;

    HttpResponse::Ok().json(json!(web::Json(deleted)))
}
