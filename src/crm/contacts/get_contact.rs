use crate::service::req_client::req_client;
use crate::types::Response;

use crate::service::toss_request::TossKindOr;
use crate::service::toss_request::{toss_request, RequestKinds};
use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::json;

use super::_types::Contact;

pub async fn send_request(req: &HttpRequest, contact_id: &str) -> Response<Contact> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/contacts/{contact_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Contact>>().await;

    return response.unwrap();
}

pub async fn get_contact(
    req: HttpRequest,
    payload: web::Path<String>,
    redis: web::Data<redis::Client>,
) -> HttpResponse {
    let contact_id = payload.into_inner();
    let response = send_request(&req, &contact_id).await;
    let contact = response.data.as_ref().expect("Contact is None");

    println!("Contact: {:#?}", contact);
    let futures = FuturesUnordered::new();

    let mut company = None;
    let mut lead = None;

    if let Some(company_id) = &contact.company_id {
        let request = toss_request(
            &req,
            company_id.clone(),
            RequestKinds::COMPANY,
            redis.clone(),
        );

        futures.push(request)
    }

    if let Some(lead_id) = &contact.lead_id {
        let request = toss_request(&req, lead_id.clone(), RequestKinds::LEAD, redis.clone());

        futures.push(request)
    }

    futures
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .for_each(|(value, kind)| match kind {
            RequestKinds::LEAD => lead = Some(TossKindOr::lead(value).expect("Lead is None")),
            RequestKinds::COMPANY => {
                company = Some(TossKindOr::company(value).expect("Company is None"))
            }
            _ => (),
        });

    let formatted = contact.format_one((company.unwrap_or_default(), lead.unwrap_or_default()));

    HttpResponse::Ok().json(json!(web::Json(formatted)))
}
