use super::_types::{Company, CompanyFormattedList};
use crate::types::Response;

use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

pub async fn get_companies(req: HttpRequest) -> HttpResponse {
    let query = [("limit", "100")];

    let response = req_client(&req)
        .get("https://unify.apideck.com/crm/companies")
        .query(&query)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json::<Response<Vec<Company>>>()
        .await
        .unwrap();

    let companies: Vec<CompanyFormattedList> = response
        .data
        .unwrap()
        .into_iter()
        .map(|company| company.format_list())
        .collect();

    HttpResponse::Ok().json(json!(web::Json(companies)))
}
