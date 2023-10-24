use super::company_types::Company;
use super::formatters::format_company;
use crate::prism_crm::users::get_user;
use crate::service::header_management::get_auth_headers;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
use serde_json::json;

pub async fn send_request(req: &HttpRequest, company_id: &String) -> Response<Company> {
    let client = Client::default();
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(&req.headers());

    let response = client
        .get(format!(
            "https://unify.apideck.com/crm/companies/{}",
            company_id
        ))
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send()
        .await;

    return response.unwrap().json::<Response<Company>>().await.unwrap();
}

pub async fn get_company(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let company_id = path.into_inner();
    let response = send_request(&req, &company_id).await;
    let company = response.data.clone().unwrap();
    let owner_id = company.owner_id.as_ref().unwrap();

    let crm_user = get_user::send_request(&req, &owner_id).await;
    let formatted_company = format_company(&company, crm_user.data.clone());

    HttpResponse::Ok().json(json!(web::Json(formatted_company)))
}
