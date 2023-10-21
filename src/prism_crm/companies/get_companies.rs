use super::company_types::Company;
use crate::prism_crm::users::get_user;
use crate::service::header_management::get_auth_headers;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
use serde_json::json;
use std::vec;

pub async fn send_request(req: &HttpRequest) -> Response<Vec<Company>> {
    let client = Client::default();
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(&req.headers());

    let response = client
        .get("https://unify.apideck.com/crm/companies") // <- Create request builder
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send() // <- Send http request
        .await;

    response
        .unwrap()
        .json::<Response<Vec<Company>>>()
        .await
        .unwrap()
}

pub async fn get_companies(req: HttpRequest) -> HttpResponse {
    let response = send_request(&req).await;
    let crm_user = get_user::send_request(&req, "512011392").await;

    let mut companies: Vec<Company> = vec![];

    response.data.unwrap().into_iter().for_each(|company| {
        let mut company = company;
        let crm_user = crm_user.data.clone().unwrap();
        company.owner = Some(crm_user);

        companies.push(company);
    });

    HttpResponse::Ok().json(json!(web::Json(companies)))
}
