use super::_types::{User, UserFormattedList};
use crate::types::Response;

use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

pub async fn get_users(req: HttpRequest) -> HttpResponse {
    let query = [("limit", "20")];

    let response = req_client(&req)
        .get("https://unify.apideck.com/crm/users")
        .query(&query)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json::<Response<Vec<User>>>()
        .await
        .unwrap();

    let users: Vec<UserFormattedList> = response
        .data
        .unwrap()
        .into_iter()
        .map(|user| user.format_list())
        .collect();

    HttpResponse::Ok().json(json!(web::Json(users)))
}
