use actix_web::{web, HttpResponse};

use awc::Client;
use serde::Deserialize;
use serde_json::json;

// #[derive(Serialize)]
// struct Info {
//     app_id: String,
//     api_key: String,
// }

#[derive(Deserialize, Debug)]
pub struct Jwks {
    pub headers: (),
}

pub async fn get_companies() -> HttpResponse {
    let client = Client::default();
    let api_key = std::env::var("APIDECK_API_KEY").unwrap();

    let res = client
        .get("https://unify.apideck.com/hris/companies") // <- Create request builder
        .insert_header(("User-Agent", "Actix-web"))
        .send() // <- Send http request
        .await;

    let data = json!({
        "status": "ok",
        "data": web::Json(res.unwrap().json::<serde_json::Value>().await.unwrap())
    });

    HttpResponse::Ok().json(data)
}
