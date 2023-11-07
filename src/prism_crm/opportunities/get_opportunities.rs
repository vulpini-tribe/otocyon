use crate::types::Response;

use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use std::vec;

use super::opp_types::{Opportunity, OpportunityFormattedList};

pub async fn send_request(req: &HttpRequest) -> Response<Vec<Opportunity>> {
    let client = req_client(req);
    let url = format!("https://unify.apideck.com/crm/opportunities?limit=10");

    let response = client.get(url).send().await;
    let response = response.unwrap().json::<Response<Vec<Opportunity>>>().await;

    return response.unwrap();
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
