use crate::service::header_management::get_auth_headers;
use crate::types::Response;

use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
use serde_json::json;
use std::vec;

use super::opp_types::{Opportunity, OpportunityFormattedList};

pub async fn send_request(req: &HttpRequest) -> Response<Vec<Opportunity>> {
    let client = Client::default();
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(&req.headers());

    let response = client
        .get("https://unify.apideck.com/crm/opportunities?limit=10")
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send()
        .await;

    response
        .unwrap()
        .json::<Response<Vec<Opportunity>>>()
        .await
        .unwrap()
}

pub async fn get_opportunities(req: HttpRequest) -> HttpResponse {
    let response = send_request(&req).await;

    let mut opportunities: Vec<OpportunityFormattedList> = vec![];
    let main_response = response.data.clone().unwrap();

    main_response.into_iter().for_each(|opportunity| {
        let formatted = opportunity.format_list();

        opportunities.push(formatted);
    });

    HttpResponse::Ok().json(json!(web::Json(opportunities)))
}
