use super::company_types::{Company, CompanyFormatted};
use super::formatters::format_company;
use crate::prism_crm::users::get_user;
use crate::prism_crm::users::user_types::CrmUser;
use crate::service::header_management::get_auth_headers;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
use futures::stream::{FuturesUnordered, StreamExt};
use serde_json::json;
use std::collections::{HashMap, HashSet};
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

    let mut companies: Vec<CompanyFormatted> = vec![];
    let mut uniq_owner_ids: HashSet<String> = HashSet::new();
    let mut owner_map: HashMap<String, CrmUser> = HashMap::new();
    let main_response = response.data.clone().unwrap();

    main_response.into_iter().for_each(|company| {
        let company = company;
        uniq_owner_ids.insert(company.owner_id.unwrap());
    });

    let futures = FuturesUnordered::new();

    for owner_id in &uniq_owner_ids {
        let crm_user = get_user::send_request(&req, &owner_id);
        futures.push(crm_user);
    }

    let _results: Vec<_> = futures.collect().await;
    _results.into_iter().for_each(|user| {
        let user = user.data.unwrap();
        owner_map.insert(user.id.clone(), user);
    });

    response.data.unwrap().into_iter().for_each(|company| {
        let owner_id = company.owner_id.clone().unwrap();
        let crm_user = owner_map.get(&owner_id);
        let formatted_company = format_company(&company, crm_user.cloned());

        companies.push(formatted_company);
    });

    HttpResponse::Ok().json(json!(web::Json(companies)))
}
