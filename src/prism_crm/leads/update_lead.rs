use super::_types::PostLead;
use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::Value;

pub async fn update_lead(req: HttpRequest, payload: web::Json<PostLead>) -> HttpResponse {
    let lead_id = req.match_info().get("lead_id").unwrap();
    let url = format!("https://unify.apideck.com/crm/leads/{lead_id}");

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
