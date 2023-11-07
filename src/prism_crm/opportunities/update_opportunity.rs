use super::opp_types::PostOpportunity;
use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::Value;

pub async fn send_request(
    req: &HttpRequest,
    payload: &PostOpportunity,
    opportunity_id: &str,
) -> Value {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/opportunities/{opportunity_id}");

    let response = client.patch(url).send_json(payload).await;
    let response = response.unwrap().json::<Value>().await;

    return response.unwrap();
}

pub async fn update_opportunity(
    req: HttpRequest,
    payload: web::Json<PostOpportunity>,
) -> HttpResponse {
    let opportunity_id = req.match_info().get("opportunity_id").unwrap();
    let main_request = send_request(&req, &payload, opportunity_id).await;

    HttpResponse::Ok().json(web::Json(main_request))
}
