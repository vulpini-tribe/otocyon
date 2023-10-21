use super::types::CrmUser;
use crate::service::header_management::get_headers;
use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
extern crate url_match;
use serde::{Deserialize, Serialize};
use serde_json::json;
use url_match::matcher;
#[derive(Debug, Serialize, Deserialize)]
struct Response {
    data: Option<CrmUser>,
    status_code: Option<u16>,
    status: Option<String>,
}

pub async fn get_user(req: HttpRequest, payload: web::Path<String>) -> HttpResponse {
    let client = Client::default();
    let company_id = payload.into_inner();
    let (app_id, auth, consumer_id, service_id) = get_headers(&req.headers());

    let tttttt = matcher("/prism/users/:user_id", req.path());

    tttttt.clone().into_iter().for_each(|key| {
        println!("{:?}", key);
    });

    match tttttt {
        Some(_) => println!("The URIs are equivalent!"),
        None => println!("The URIs are not equivalent!"),
    }

    let response = client
        .get(format!(
            "https://unify.apideck.com/crm/users/{}",
            company_id
        ))
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send()
        .await;

    let response = response.unwrap().json::<Response>().await.unwrap();

    HttpResponse::Ok().json(json!(web::Json(response)))
}
