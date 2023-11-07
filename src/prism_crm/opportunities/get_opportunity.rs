use super::opp_types::Opportunity;
use crate::companies::company_types::Company;
use crate::prism_crm::contacts::contact_types::Contact;
use crate::prism_crm::leads::lead_types::Lead;
use crate::prism_crm::pipelines::pipeline_types::Pipeline;

use crate::service::req_client::req_client;
use crate::service::toss_request::{toss_request, RequestKinds};
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use futures::StreamExt;
use serde_json::{json, Value};

use futures::stream::FuturesUnordered;
use std::sync::{Arc, Mutex};

pub async fn send_request(req: &HttpRequest, opportunity_id: &str) -> Response<Value> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/opportunities/{opportunity_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Value>>().await;

    return response.unwrap();
}

pub fn to_opportunity(value: Value) -> Opportunity {
    serde_json::from_value(value).unwrap()
}

pub fn to_lead(value: Response<Value>) -> Lead {
    let value = value.data.into();
    serde_json::from_value(value).unwrap()
}

pub fn to_pipeline(value: Response<Value>) -> Pipeline {
    let value = value.data.into();
    serde_json::from_value(value).unwrap()
}

pub fn to_company(value: Response<Value>) -> Company {
    let value = value.data.into();
    serde_json::from_value(value).unwrap()
}

pub fn fmt_to_contact(value: Response<Value>) -> Contact {
    let value = value.data.into();
    serde_json::from_value(value).unwrap()
}

pub async fn get_opportunity(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let opportunity_id = path.into_inner();
    let response = send_request(&req, &opportunity_id).await;
    let opportunity = response.data.clone().unwrap();
    let opportunity = to_opportunity(opportunity);

    let futures = FuturesUnordered::new();

    let mut company: Option<Company> = None;
    let mut contact: Option<Contact> = None;
    let mut lead: Option<Lead> = None;
    let mut pipeline: Option<Pipeline> = None;

    if opportunity.company_id.is_some() {
        let company_id = opportunity.company_id.clone().unwrap();
        let company_url = Arc::new(Mutex::new(format!(
            "https://unify.apideck.com/crm/companies/{company_id}"
        )));

        futures.push(toss_request(&req, company_url, RequestKinds::COMPANY));
    }

    if opportunity.contact_id.is_some() {
        let contact_id = opportunity.contact_id.clone().unwrap();
        let contact_url = Arc::new(Mutex::new(format!(
            "https://unify.apideck.com/crm/contacts/{contact_id}"
        )));

        futures.push(toss_request(&req, contact_url, RequestKinds::CONTACT));
    }

    if opportunity.lead_id.is_some() {
        let lead_id = opportunity.lead_id.clone().unwrap();
        let lead_url = Arc::new(Mutex::new(format!(
            "https://unify.apideck.com/crm/leads/{lead_id}"
        )));

        futures.push(toss_request(&req, lead_url, RequestKinds::LEAD));
    }

    if opportunity.pipeline_id.is_some() {
        let pipeline_id = opportunity.pipeline_id.clone().unwrap();
        let pipeline_url = Arc::new(Mutex::new(format!(
            "https://unify.apideck.com/crm/pipelines/{pipeline_id}"
        )));

        futures.push(toss_request(&req, pipeline_url, RequestKinds::PIPELINE));
    }

    futures
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .for_each(|(value, kind)| match kind {
            RequestKinds::PIPELINE => pipeline = Some(to_pipeline(value)),
            RequestKinds::LEAD => lead = Some(to_lead(value)),
            RequestKinds::COMPANY => company = Some(to_company(value)),
            RequestKinds::CONTACT => contact = Some(fmt_to_contact(value)),
        });

    let company = match company {
        Some(data) => Ok(data),
        None => Err(()),
    };

    let pipeline = match pipeline {
        Some(data) => Ok(data),
        None => Err(()),
    };

    let lead = match lead {
        Some(data) => Ok(data),
        None => Err(()),
    };

    let contact = match contact {
        Some(data) => Ok(data),
        None => Err(()),
    };

    let formatted = opportunity.format_one((
        company.unwrap_or_default(),
        pipeline.unwrap_or_default(),
        lead.unwrap_or_default(),
        contact.unwrap_or_default(),
    ));

    HttpResponse::Ok().json(json!(web::Json(formatted)))
}
