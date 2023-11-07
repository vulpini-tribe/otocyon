use super::_types::{Lead, LeadFormattedList};
use crate::types::Response;

use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

pub async fn get_leads(req: HttpRequest) -> HttpResponse {
    let query = [("limit", "20")];

    let response = req_client(&req)
        .get("https://unify.apideck.com/crm/leads")
        .query(&query)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json::<Response<Vec<Lead>>>()
        .await
        .unwrap();

    let contacts: Vec<LeadFormattedList> = response
        .data
        .unwrap()
        .into_iter()
        .map(|contact| contact.format_list())
        .collect();

    HttpResponse::Ok().json(json!(web::Json(contacts)))
}
