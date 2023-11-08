use super::_types::PostUser;
use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::Value;

pub async fn update_user(req: HttpRequest, payload: web::Json<PostUser>) -> HttpResponse {
    let user_id = req.match_info().get("user_id").unwrap();
    let url = format!("https://unify.apideck.com/crm/pipelines/{user_id}");

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
