use super::opp_types::Opportunity;
use crate::prism_crm::companies::get_company;
use crate::prism_crm::contacts::get_contact;
use crate::prism_crm::leads::get_lead;
use crate::prism_crm::pipelines::get_pipeline;

use crate::service::header_management::get_auth_headers;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
use serde_json::json;

pub async fn send_request(req: &HttpRequest, opportunity_id: &str) -> Response<Opportunity> {
    let client = Client::default();
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(&req.headers());

    let response = client
        .get(format!(
            "https://unify.apideck.com/crm/opportunities/{opportunity_id}"
        ))
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send()
        .await;

    return response
        .unwrap()
        .json::<Response<Opportunity>>()
        .await
        .unwrap();
}

pub async fn get_opportunity(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let opportunity_id = path.into_inner();
    let response = send_request(&req, &opportunity_id).await;
    let opportunity = response.data.clone().unwrap();

    let company_id = String::from("17049828698");
    let company = get_company::send_request(&req, &company_id).await;

    let contact_id = String::from("14856");
    let contact = get_contact::send_request(&req, &contact_id).await;

    let lead_id = String::from("14856");
    let lead = get_lead::send_request(&req, &lead_id).await;

    let pipeline_id = String::from("default");
    let pipeline = get_pipeline::send_request(&req, &pipeline_id).await;

    let external_values = (company.data, contact.data, pipeline.data, lead.data);

    let formatted = opportunity.format(external_values);

    HttpResponse::Ok().json(json!(web::Json(formatted)))
}
