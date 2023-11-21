use crate::service::req_client::req_client;
use crate::types::Response;

use crate::service::toss_request::TossKindOr;
use crate::service::toss_request::{toss_request, RequestKinds};
use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::json;

use super::_types::Lead;

pub async fn send_request(req: &HttpRequest, lead_id: &str) -> Response<Lead> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/leads/{lead_id}");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Lead>>().await;

    return response.unwrap();
}

pub async fn get_lead(
    req: HttpRequest,
    payload: web::Path<String>,
    redis: web::Data<redis::Client>,
) -> HttpResponse {
    let lead_id = payload.into_inner();
    let response = send_request(&req, &lead_id).await;
    let lead = response.data.as_ref().expect("Lead is None");

    let futures = FuturesUnordered::new();

    let mut company = None;

    if let Some(company_id) = &lead.company_id {
        let request = toss_request(
            &req,
            company_id.clone(),
            RequestKinds::COMPANY,
            redis.clone(),
        );

        futures.push(request)
    }

    futures
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .for_each(|(value, kind)| match kind {
            RequestKinds::COMPANY => {
                company = Some(TossKindOr::company(value).expect("Company is None"))
            }
            _ => (),
        });

    let formatted = lead.format_one(company.unwrap_or_default());

    HttpResponse::Ok().json(json!(web::Json(formatted)))
}
