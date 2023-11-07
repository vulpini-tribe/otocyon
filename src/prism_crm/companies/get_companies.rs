use super::company_types::{Company, CompanyFormattedForList};
use super::formatters::format_company_list;
use crate::types::Response;

use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use std::collections::HashSet;
use std::vec;

pub async fn send_request(req: &HttpRequest) -> Response<Vec<Company>> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/companies?limit=10");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Vec<Company>>>().await;

    return response.unwrap();
}

pub async fn get_companies(req: HttpRequest) -> HttpResponse {
    let response = send_request(&req).await;

    let mut companies: Vec<CompanyFormattedForList> = vec![];
    let mut uniq_owner_ids: HashSet<String> = HashSet::new();
    let main_response = response.data.clone().unwrap();

    main_response.into_iter().for_each(|company| {
        let company = company;
        uniq_owner_ids.insert(company.owner_id.unwrap());
    });

    response.data.unwrap().into_iter().for_each(|company| {
        let formatted_company = format_company_list(&company);

        companies.push(formatted_company);
    });

    HttpResponse::Ok().json(json!(web::Json(companies)))
}
