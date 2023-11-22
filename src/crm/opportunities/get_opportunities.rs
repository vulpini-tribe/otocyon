use super::_types::{Opportunity, OpportunityFormattedList};
use crate::types::Response;

use crate::service::redix;
use crate::service::req_client::req_client;
use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub async fn get_opportunities(req: HttpRequest, redis: web::Data<redis::Client>) -> HttpResponse {
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

    let futures = FuturesUnordered::new();
    let mut stage_keys = HashSet::new();
    let mut stage_names = HashMap::new();
    let opportunities = response.data.unwrap_or_default().into_iter();

    opportunities.clone().into_iter().for_each(|opportunity| {
        stage_keys.insert(format!(
            "{}.{}",
            opportunity.pipeline_id.unwrap(),
            opportunity.pipeline_stage_id.unwrap()
        ));
    });

    for stage_id in stage_keys {
        let stage_id = Arc::new(stage_id);

        let request = redix::get_one(&redis, stage_id.clone());
        futures.push(request)
    }

    futures
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .for_each(|result| {
            let (key, value) = result.unwrap();

            stage_names.insert(key, value);
        });

    let opportunities: Vec<OpportunityFormattedList> = opportunities
        .clone()
        .map(|opportunity| {
            let pipeline_stage_id = opportunity
                .pipeline_stage_id
                .clone()
                .unwrap_or("".to_owned());

            let stage_id = format!(
                "{}.{}",
                opportunity.pipeline_id.clone().unwrap_or("".to_owned()),
                pipeline_stage_id,
            );

            match stage_names.get(&stage_id) {
                Some(name) => opportunity.format_list(name.to_owned()),
                None => opportunity.format_list(pipeline_stage_id),
            }
        })
        .collect();

    HttpResponse::Ok().json(json!(web::Json(opportunities)))
}
