use super::opp_types::Opportunity;
use crate::companies::company_types::Company;
use crate::prism_crm::companies::get_company;
use crate::prism_crm::contacts::contact_types::Contact;
use crate::prism_crm::contacts::get_contact;
use crate::prism_crm::leads::get_lead;
use crate::prism_crm::leads::lead_types::Lead;
use crate::prism_crm::pipelines::get_pipeline;
use crate::prism_crm::pipelines::pipeline_types::Pipeline;

use crate::service::header_management::get_auth_headers;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::{json, Value};

use awc::Client;

use futures::stream;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum RequestKinds {
    // COMPANY,
    LEAD,
    CONTACT,
    PIPELINE,
}

pub async fn send_request(req: &HttpRequest, opportunity_id: &str) -> Response<Opportunity> {
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(&req.headers());

    let client = Client::default();
    let url = format!("https://unify.apideck.com/crm/opportunities/{opportunity_id}");

    let response = client
        .get(url)
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send()
        .await;

    let response = response.unwrap().json::<Response<Opportunity>>().await;

    return response.unwrap();
}

pub async fn send_request_gen(
    req: &HttpRequest,
    url: Arc<Mutex<String>>,
    kind: RequestKinds,
) -> (Value, RequestKinds) {
    let url = url.lock().unwrap().clone();
    let client = Client::default();
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(&req.headers());

    let response = client
        .get(url)
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send()
        .await;

    let response = response.unwrap().json::<Value>().await.unwrap();

    (response, kind)
}

pub fn to_opportunity(value: Value) -> Opportunity {
    serde_json::from_value(value).unwrap()
}

pub fn to_lead(value: Value) -> Lead {
    serde_json::from_value(value).unwrap()
}

pub fn to_pipeline(value: Value) -> Pipeline {
    serde_json::from_value(value).unwrap()
}

pub fn to_company(value: Value) -> Company {
    serde_json::from_value(value).unwrap()
}

pub fn to_contact(value: Value) -> Contact {
    serde_json::from_value(value).unwrap()
}

pub async fn get_opportunity(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let opportunity_id = path.into_inner();
    let response = send_request(&req, &opportunity_id).await;
    let opportunity = response.data.clone().unwrap();

    let futures = FuturesUnordered::new();

    // let company_id = String::from("17049828698");
    // let company_url = Arc::new(Mutex::new(format!(
    //     "https://unify.apideck.com/crm/companies/{company_id}"
    // )));

    let contact_id = String::from("14856");
    let contact_url = Arc::new(Mutex::new(format!(
        "https://unify.apideck.com/crm/contacts/{contact_id}"
    )));

    let lead_id = String::from("14856");
    let lead_url = Arc::new(Mutex::new(format!(
        "https://unify.apideck.com/crm/leads/{lead_id}"
    )));

    let pipeline_id = String::from("default");
    let pipeline_url = Arc::new(Mutex::new(format!(
        "https://unify.apideck.com/crm/pipelines/{pipeline_id}"
    )));

    // futures.push(send_request_gen(&req, company_url, RequestKinds::COMPANY));
    futures.push(send_request_gen(&req, contact_url, RequestKinds::CONTACT));
    futures.push(send_request_gen(&req, lead_url, RequestKinds::LEAD));
    futures.push(send_request_gen(&req, pipeline_url, RequestKinds::PIPELINE));

    let results: Vec<_> = futures.collect().await;

    let mut pipeline: Option<Pipeline> = None;
    let mut lead: Option<Lead> = None;
    // let mut company = None;
    let mut contact = None;

    results.into_iter().for_each(|(value, kind)| match kind {
        RequestKinds::PIPELINE => {
            pipeline = Some(to_pipeline(value));
        }
        RequestKinds::LEAD => {
            lead = Some(to_lead(value));
        }
        // RequestKinds::COMPANY => {
        //     company = Some(to_company(value));
        // }
        RequestKinds::CONTACT => {
            contact = Some(to_contact(value));
        }
    });

    // println!("PIPELINE: {:#?}", pipeline);
    // println!("LEAD: {:#?}", lead);
    // println!("CONTACT: {:#?}", contact);
    // println!("COMPANY: {:#?}", company);

    let pipeline_data = match pipeline {
        Some(data) => Ok(data.data),
        None => Err(()),
    };

    let lead_data = match lead {
        Some(data) => Ok(data.data),
        None => Err(()),
    };

    let contact_data = match contact {
        Some(data) => Ok(data.data),
        None => Err(()),
    };

    let external_values = (
        pipeline_data.unwrap(),
        lead_data.unwrap(),
        contact_data.unwrap(),
    );

    let formatted = opportunity.format(external_values);

    HttpResponse::Ok().json(json!(web::Json(formatted)))
}
