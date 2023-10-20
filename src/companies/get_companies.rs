use crate::service::header_management::get_headers;
use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
use serde_json::json;

pub async fn get_companies(req: HttpRequest) -> HttpResponse {
    let client = Client::default();
    let (app_id, auth, consumer_id, service_id) = get_headers(&req.headers());

    // send request to apideck
    let res = client
        .get("https://unify.apideck.com/crm/companies") // <- Create request builder
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send() // <- Send http request
        .await;

    // send request to get owner_id by its id (CRM users DB)

    let data = json!({
        "status": "ok",
        "data": web::Json(res.unwrap().json::<serde_json::Value>().await.unwrap())
    });

    HttpResponse::Ok().json(data)
}
