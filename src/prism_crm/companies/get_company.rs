use super::company_types::Company;
use super::formatters::format_company_extended;
use crate::prism_crm::users::get_user;
use crate::service::req_client::req_client;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

pub async fn send_request(req: &HttpRequest, company_id: &str) -> Response<Company> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/companies/{company_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Company>>().await;

    return response.unwrap();
}

pub async fn get_company(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let company_id = path.into_inner();
    let response = send_request(&req, &company_id).await;
    let company = response.data.clone().unwrap();
    let owner_id = company.owner_id.as_ref().unwrap();

    let crm_user = get_user::send_request(&req, &owner_id).await;
    let formatted_company = format_company_extended(&company, crm_user.data.clone());

    HttpResponse::Ok().json(json!(web::Json(formatted_company)))
}
