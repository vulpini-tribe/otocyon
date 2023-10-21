use crate::prism_crm::users;
use crate::service::header_management::get_headers;
use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Company {
    id: String,
    name: String,
    owner_id: String,
    owner: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    data: Vec<Company>,
}

pub async fn get_companies(req: HttpRequest) -> HttpResponse {
    let client = Client::default();
    let (app_id, auth, consumer_id, service_id) = get_headers(&req.headers());

    // send request to apideck
    let response = client
        .get("https://unify.apideck.com/crm/companies") // <- Create request builder
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send() // <- Send http request
        .await;

    let res = response.unwrap().json::<Response>().await.unwrap();
    // send request to get owner_id by its id (CRM users DB)
    let path_string = "512011392".to_string();
    let path_value = web::Path::from(path_string);

    let _crm_user = users::get_user::get_user(req, path_value).await;
    // let test = crm_user.body().to_owned();
    // get data from test
    // let test_data = test.unwrap().into_inner();
    // println!("{:?}", crm_user.body());
    // println!("{:?}", test);

    // format received data to one unified format
    HttpResponse::Ok().json(json!(web::Json(res)))
}
