use super::_types::{Contact, ContactFormattedList};
use crate::types::Response;

use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

pub async fn get_contacts(req: HttpRequest) -> HttpResponse {
    let query = [("limit", "50")];

    let response = req_client(&req)
        .get("https://unify.apideck.com/crm/contacts")
        .query(&query)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json::<Response<Vec<Contact>>>()
        .await
        .unwrap();

    let contacts: Vec<ContactFormattedList> = response
        .data
        .unwrap()
        .into_iter()
        .map(|contact| contact.format_list())
        .collect();

    HttpResponse::Ok().json(json!(web::Json(contacts)))
}
