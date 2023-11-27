use super::_types::User;
use crate::service::req_client::req_client;
use crate::types::Response;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

pub async fn send_request(req: &HttpRequest, user_id: &str) -> Response<User> {
    let client = req_client(req);

    // println!("User ID: {:#?}", user_id);
    let url = format!("https://unify.apideck.com/crm/users/{user_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<User>>().await;

    // println!("Response: {:#?}", response);

    return response.unwrap();
}

pub async fn get_user(req: HttpRequest, payload: web::Path<String>) -> HttpResponse {
    let user_id = payload.into_inner();
    let response = send_request(&req, &user_id).await;
    let user = response.data.as_ref().expect("User is unknown");

    let formatted = user.format_one();

    HttpResponse::Ok().json(json!(web::Json(formatted)))
}
