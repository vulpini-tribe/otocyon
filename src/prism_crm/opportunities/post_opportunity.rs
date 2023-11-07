use super::opp_types::PostOpportunity;
use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::Value;

pub async fn send_request(req: &HttpRequest, payload: &PostOpportunity) -> Value {
    let client = req_client(req);
    let url = "https://unify.apideck.com/crm/opportunities";

    let response = client.post(url).send_json(payload).await;
    let response = response.unwrap().json::<Value>().await;

    return response.unwrap();
}

pub async fn post_opportunity(
    req: HttpRequest,
    payload: web::Json<PostOpportunity>,
) -> HttpResponse {
    let main_request = send_request(&req, &payload).await;

    HttpResponse::Ok().json(web::Json(main_request))
}
