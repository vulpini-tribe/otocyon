use super::_types::PostActivity;
use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::Value;

pub async fn update_activity(req: HttpRequest, payload: web::Json<PostActivity>) -> HttpResponse {
    let activity_id = req.match_info().get("activity_id").unwrap();
    let url = format!("https://unify.apideck.com/crm/activities/{activity_id}");

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
