use super::_types::Note;
use crate::service::req_client::req_client;
use crate::types::Response;

use crate::service::toss_request::TossKindOr;
use crate::service::toss_request::{toss_request, RequestKinds};
use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::json;

pub async fn send_request(req: &HttpRequest, note_id: &str) -> Response<Note> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/notes/{note_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Note>>().await;

    return response.unwrap();
}

pub async fn get_note(
    req: HttpRequest,
    path: web::Path<String>,
    redis: web::Data<redis::Client>,
) -> HttpResponse {
    let note_id = path.into_inner();
    let note = send_request(&req, &note_id).await;
    let note = note.data.as_ref().unwrap();

    let futures = FuturesUnordered::new();

    let company_id = note.company_id.clone().unwrap_or(String::from(""));
    let contact_id = note.contact_id.clone().unwrap_or(String::from(""));
    let opportunity_id = note.opportunity_id.clone().unwrap_or(String::from(""));

    let mut owner = None;
    let mut company = None;
    let mut contact = None;
    let mut opportunity = None;
    let mut lead = None;

    if let Some(owner_id) = &note.owner_id {
        let request = toss_request(&req, owner_id.clone(), RequestKinds::USER, redis.clone());

        futures.push(request)
    }

    if company_id.len() > 0 && company_id != "n/a" {
        let request = toss_request(
            &req,
            company_id.clone(),
            RequestKinds::COMPANY,
            redis.clone(),
        );

        futures.push(request)
    }

    if contact_id.len() > 0 && contact_id != "n/a" {
        let request = toss_request(&req, contact_id, RequestKinds::CONTACT, redis.clone());

        futures.push(request)
    }

    if opportunity_id.len() > 0 && opportunity_id != "n/a" {
        let request = toss_request(
            &req,
            opportunity_id,
            RequestKinds::OPPORTUNITY,
            redis.clone(),
        );

        futures.push(request)
    }

    if let Some(lead_id) = &note.lead_id {
        let request = toss_request(&req, lead_id.clone(), RequestKinds::LEAD, redis.clone());

        futures.push(request)
    }

    futures
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .for_each(|(value, kind)| match kind {
            RequestKinds::USER => owner = Some(TossKindOr::user(value).expect("Owner is None")),
            RequestKinds::COMPANY => {
                company = Some(TossKindOr::company(value).expect("Company is None"))
            }
            RequestKinds::CONTACT => {
                contact = Some(TossKindOr::contact(value).expect("Contact is None"))
            }
            RequestKinds::OPPORTUNITY => {
                opportunity = Some(TossKindOr::opportunity(value).expect("Opportunity is None"))
            }
            RequestKinds::LEAD => lead = Some(TossKindOr::lead(value).expect("Lead is None")),
            _ => (),
        });

    let formatted = note.format_one((owner, company, contact, opportunity, lead));

    HttpResponse::Ok().json(json!(web::Json(formatted)))
}
