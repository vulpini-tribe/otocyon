use crate::types::Response;

use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

use super::_types::{Activity, ActivityFormattedList};

pub async fn get_activities(req: HttpRequest) -> HttpResponse {
    let query = [("limit", "20")];

    let response = req_client(&req)
        .get("https://unify.apideck.com/crm/activities")
        .query(&query)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json::<Response<Vec<Activity>>>()
        .await
        .unwrap();

    let activities: Vec<ActivityFormattedList> = response
        .data
        .unwrap()
        .into_iter()
        .map(|opportunity| opportunity.format_list())
        .collect();

    HttpResponse::Ok().json(json!(web::Json(activities)))
}
