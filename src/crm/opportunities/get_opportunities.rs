use crate::types::Response;

use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

use super::_types::{Opportunity, OpportunityFormattedList};

pub async fn get_opportunities(req: HttpRequest) -> HttpResponse {
    let query = [("limit", "100")];

    let response = req_client(&req)
        .get("https://unify.apideck.com/crm/opportunities")
        .query(&query)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json::<Response<Vec<Opportunity>>>()
        .await
        .unwrap();

    let opportunities: Vec<OpportunityFormattedList> = response
        .data
        .unwrap()
        .into_iter()
        .map(|opportunity| opportunity.format_list())
        .collect();

    HttpResponse::Ok().json(json!(web::Json(opportunities)))
}
