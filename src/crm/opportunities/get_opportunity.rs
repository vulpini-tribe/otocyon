use super::_types::Opportunity;
use crate::service::req_client::req_client;
use crate::service::toss_request::{toss_request, RequestKinds};
use crate::types::Response;

use crate::service::toss_request::TossKindOr;
use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::json;

pub async fn send_request(req: &HttpRequest, opportunity_id: &str) -> Response<Opportunity> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/opportunities/{opportunity_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Opportunity>>().await;

    return response.unwrap();
}

pub async fn get_opportunity(
    req: HttpRequest,
    path: web::Path<String>,
    redis: web::Data<redis::Client>,
) -> HttpResponse {
    let opportunity_id = path.into_inner();
    let opportunity = send_request(&req, &opportunity_id).await;
    let opportunity = opportunity.data.as_ref().unwrap();

    let futures = FuturesUnordered::new();

    let mut lead = None;
    let mut company = None;
    let mut contact = None;
    let mut pipeline = None;

    if let Some(company_id) = &opportunity.company_id {
        let request = toss_request(
            &req,
            company_id.clone(),
            RequestKinds::COMPANY,
            redis.clone(),
        );

        futures.push(request)
    }

    if opportunity.primary_contact_id.len() > 0 {
        let request = toss_request(
            &req,
            opportunity.primary_contact_id.clone(),
            RequestKinds::CONTACT,
            redis.clone(),
        );

        futures.push(request)
    }

    if let Some(lead_id) = &opportunity.lead_id {
        let request = toss_request(&req, lead_id.clone(), RequestKinds::LEAD, redis.clone());

        futures.push(request)
    }

    if let Some(pipeline_id) = &opportunity.pipeline_id {
        let request = toss_request(
            &req,
            pipeline_id.clone(),
            RequestKinds::PIPELINE,
            redis.clone(),
        );

        futures.push(request)
    }

    futures
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .for_each(|(value, kind)| match kind {
            RequestKinds::PIPELINE => pipeline = Some(TossKindOr::pipeline(value).unwrap()),
            RequestKinds::LEAD => lead = Some(TossKindOr::lead(value).unwrap()),
            RequestKinds::COMPANY => company = Some(TossKindOr::company(value).unwrap()),
            RequestKinds::CONTACT => contact = Some(TossKindOr::contact(value).unwrap()),
        });

    let formatted = opportunity.format_one((
        company.unwrap_or_default(),
        pipeline.unwrap_or_default(),
        lead.unwrap_or_default(),
        contact.unwrap_or_default(),
    ));

    HttpResponse::Ok().json(json!(web::Json(formatted)))
}
