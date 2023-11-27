use super::_types::Activity;
use crate::service::req_client::req_client;
use crate::service::toss_request::TossKindOr;
use crate::service::toss_request::{toss_request, RequestKinds};
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::json;

pub async fn send_request(req: &HttpRequest, activity_id: &str) -> Response<Activity> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/activities/{activity_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Activity>>().await;

    return response.unwrap();
}

pub async fn get_activity(
    req: HttpRequest,
    path: web::Path<String>,
    redis: web::Data<redis::Client>,
) -> HttpResponse {
    let activity_id = path.into_inner();

    let activity = send_request(&req, &activity_id).await;
    let activity = activity.data.as_ref().unwrap();

    let futures = FuturesUnordered::new();

    let owner_id = activity.owner_id.clone().unwrap_or(String::from(""));
    let contact_id = activity.contact_id.clone().unwrap_or(String::from(""));
    let opportunity_id = activity.opportunity_id.clone().unwrap_or(String::from(""));

    let mut user = None;
    let mut company = None;
    let mut contact = None;
    let mut opportunity = None;

    if owner_id.len() > 0 && owner_id != "n/a" {
        let request = toss_request(&req, owner_id.clone(), RequestKinds::USER, redis.clone());

        futures.push(request)
    }

    if let Some(company_id) = &activity.company_id {
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

    futures
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .for_each(|(value, kind)| match kind {
            RequestKinds::USER => {
                let possible_user = TossKindOr::user(value);

                if !possible_user.is_none() {
                    user = possible_user
                }
            }
            RequestKinds::COMPANY => {
                let possible_company = TossKindOr::company(value);

                if !possible_company.is_none() {
                    company = possible_company
                }
            }
            RequestKinds::CONTACT => {
                let possible_contact = TossKindOr::contact(value);

                if !possible_contact.is_none() {
                    contact = possible_contact
                }
            }
            RequestKinds::OPPORTUNITY => {
                let possible_opportunity = TossKindOr::opportunity(value);

                if !possible_opportunity.is_none() {
                    opportunity = possible_opportunity
                }
            }
            _ => (),
        });

    let formatted = activity.format_one((user, company, contact, opportunity));

    HttpResponse::Ok().json(json!(web::Json(formatted)))
}
