use super::_types::DeleteCompanies;
use crate::service::req_client::req_client;

use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::json;
use web::Json;

pub async fn delete_companies(req: HttpRequest, payload: Json<DeleteCompanies>) -> HttpResponse {
    let client = req_client(&req);
    let futures = FuturesUnordered::new();

    payload.ids.iter().for_each(|company_id| {
        let url = format!("https://unify.apideck.com/crm/companies/{company_id}");
        let request = client.delete(url).send();

        futures.push(request);
    });

    futures.collect::<Vec<_>>().await;

    HttpResponse::Ok().json(json!(Json("Ok")))
}
