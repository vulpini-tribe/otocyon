use super::_types::PostOpportunity;
use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::Value;

pub async fn update_opportunity(
    req: HttpRequest,
    payload: web::Json<PostOpportunity>,
) -> HttpResponse {
    let opportunity_id = req.match_info().get("opportunity_id").unwrap();
    let url = format!("https://unify.apideck.com/crm/opportunities/{opportunity_id}");

    let response = req_client(&req)
        .patch(url)
        .send_json(&payload)
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    HttpResponse::Ok().json(web::Json(response))
}
