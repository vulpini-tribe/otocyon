use crate::service::req_client::req_client;
use crate::service::toss_request::{toss_request, RequestKinds};
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

pub async fn send_request(req: &HttpRequest, opportunity_id: &str) -> Response<Value> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/opportunities/{opportunity_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Value>>().await;

    return response.unwrap();
}

pub async fn get_opportunity(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let opportunity_id = path.into_inner();
    let opportunity = send_request(&req, &opportunity_id).await.to_opportunity();

    let futures = FuturesUnordered::new();

    let mut company = None;
    let mut contact = None;
    let mut lead = None;
    let mut pipeline = None;

    if opportunity.company_id.is_some() {
        let company_id = opportunity.company_id.clone().unwrap();
        let company_id = Arc::new(Mutex::new(company_id));

        futures.push(toss_request(&req, company_id, RequestKinds::COMPANY));
    }

    if opportunity.contact_id.is_some() {
        let contact_id = opportunity.contact_id.clone().unwrap();
        let contact_id = Arc::new(Mutex::new(contact_id));

        futures.push(toss_request(&req, contact_id, RequestKinds::CONTACT));
    }

    if opportunity.lead_id.is_some() {
        let lead_id = opportunity.lead_id.clone().unwrap();
        let lead_id = Arc::new(Mutex::new(lead_id));

        futures.push(toss_request(&req, lead_id, RequestKinds::LEAD));
    }

    if opportunity.pipeline_id.is_some() {
        let pipeline_id = opportunity.pipeline_id.clone().unwrap();
        let pipeline_id = Arc::new(Mutex::new(pipeline_id));

        futures.push(toss_request(&req, pipeline_id, RequestKinds::PIPELINE));
    }

    futures
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .for_each(|(value, kind)| match kind {
            RequestKinds::PIPELINE => pipeline = Some(value.to_pipeline()),
            RequestKinds::LEAD => lead = Some(value.to_lead()),
            RequestKinds::COMPANY => company = Some(value.to_company()),
            RequestKinds::CONTACT => contact = Some(value.to_contact()),
        });

    let formatted = opportunity.format_one((
        company.unwrap_or_default(),
        pipeline.unwrap_or_default(),
        lead.unwrap_or_default(),
        contact.unwrap_or_default(),
    ));

    HttpResponse::Ok().json(json!(web::Json(formatted)))
}
