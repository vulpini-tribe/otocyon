use super::_types::Activity;
use crate::service::req_client::req_client;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

pub async fn send_request(req: &HttpRequest, activity_id: &str) -> Response<Activity> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/activities/{activity_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Activity>>().await;

    return response.unwrap();
}

pub async fn get_activity(
    req: HttpRequest,
    path: web::Path<String>,
    // redis: web::Data<redis::Client>,
) -> HttpResponse {
    let activity_id = path.into_inner();

    let activity = send_request(&req, &activity_id).await;
    let activity = activity.data.as_ref().unwrap();

    let formatted = activity.format_one();

    HttpResponse::Ok().json(json!(web::Json(formatted)))
}
