use super::opp_types::PostOpportunity;
use crate::service::header_management::get_auth_headers;
use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
use serde_json::Value;

pub async fn send_request(
    req: &HttpRequest,
    payload: &PostOpportunity,
    opportunity_id: &str,
) -> Value {
    let client = Client::default();
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(&req.headers());

    let response = client
        .patch(format!(
            "https://unify.apideck.com/crm/opportunities/{opportunity_id}"
        ))
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send_json(payload)
        .await;

    return response.unwrap().json::<Value>().await.unwrap();
}

pub async fn update_opportunity(
    req: HttpRequest,
    payload: web::Json<PostOpportunity>,
) -> HttpResponse {
    let opportunity_id = req.match_info().get("opportunity_id").unwrap();
    let main_request = send_request(&req, &payload, opportunity_id).await;

    HttpResponse::Ok().json(web::Json(main_request))
}
