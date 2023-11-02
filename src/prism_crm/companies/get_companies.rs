use super::company_types::{Company, CompanyFormattedForList};
use super::formatters::format_company_list;
use crate::service::header_management::get_auth_headers;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
use serde_json::json;
use std::collections::HashSet;
use std::vec;

pub async fn send_request(req: &HttpRequest) -> Response<Vec<Company>> {
    let client = Client::default();
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(&req.headers());

    let response = client
        .get("https://unify.apideck.com/crm/companies?limit=10")
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send()
        .await;

    response
        .unwrap()
        .json::<Response<Vec<Company>>>()
        .await
        .unwrap()
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
