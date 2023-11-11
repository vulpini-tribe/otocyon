use crate::crm::companies::_types::Company;
use crate::service::req_client::req_client;
use crate::service::toss_request::{toss_request, RequestKinds};
use crate::types::Response;

use crate::contacts::_types::Contact;
use crate::leads::_types::Lead;
use crate::pipelines::_types::Pipeline;
use crate::service::toss_request::TypeOr;
use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::{json, Value};

pub async fn send_request(req: &HttpRequest, opportunity_id: &str) -> Response<Value> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/opportunities/{opportunity_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Value>>().await;

    return response.unwrap();
}

impl TypeOr<Response<Company>, Response<Lead>, Response<Pipeline>, Response<Contact>> {
    fn pipeline(self) -> Option<Pipeline> {
        if let TypeOr::One(c) = self {
            c.data
        } else {
            None
        }
    }

    fn company(self) -> Option<Company> {
        if let TypeOr::Left(c) = self {
            c.data
        } else {
            None
        }
    }

    fn lead(self) -> Option<Lead> {
        if let TypeOr::Right(c) = self {
            c.data
        } else {
            None
        }
    }

    fn contact(self) -> Option<Contact> {
        if let TypeOr::Two(c) = self {
            c.data
        } else {
            None
        }
    }
}

pub async fn get_opportunity(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let opportunity_id = path.into_inner();
    let opportunity = send_request(&req, &opportunity_id).await.to_opportunity();

    let futures = FuturesUnordered::new();

    let mut lead = None;
    let mut company = None;
    let mut contact = None;
    let mut pipeline = None;

    if opportunity.company_id.is_some() {
        let company_id = opportunity.company_id.clone().unwrap();

        futures.push(toss_request(&req, company_id, RequestKinds::COMPANY));
    }

    if opportunity.contact_id.is_some() {
        let contact_id = opportunity.contact_id.clone().unwrap();

        futures.push(toss_request(&req, contact_id, RequestKinds::CONTACT));
    }

    if opportunity.lead_id.is_some() {
        let lead_id = opportunity.lead_id.clone().unwrap();

        futures.push(toss_request(&req, lead_id, RequestKinds::LEAD));
    }

    if opportunity.pipeline_id.is_some() {
        let pipeline_id = opportunity.pipeline_id.clone().unwrap();

        futures.push(toss_request(&req, pipeline_id, RequestKinds::PIPELINE));
    }

    futures
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .for_each(|(value, kind)| match kind {
            RequestKinds::PIPELINE => pipeline = Some(TypeOr::pipeline(value).unwrap()),
            RequestKinds::LEAD => lead = Some(TypeOr::lead(value).unwrap()),
            RequestKinds::COMPANY => company = Some(TypeOr::company(value).unwrap()),
            RequestKinds::CONTACT => contact = Some(TypeOr::contact(value).unwrap()),
        });

    let formatted = opportunity.format_one((
        company.unwrap_or_default(),
        pipeline.unwrap_or_default(),
        lead.unwrap_or_default(),
        contact.unwrap_or_default(),
    ));

    HttpResponse::Ok().json(json!(web::Json(formatted)))
}
