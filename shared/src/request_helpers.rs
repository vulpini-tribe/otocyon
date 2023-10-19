use dotenvy::dotenv;
use reqwest::header;
use std::env;

pub fn populate_headers(service: String, consumer_id: String) -> header::HeaderMap {
    dotenv().ok();

    let api_key = env::var("APIDECK_API_KEY").expect("APIDECK_API_KEY must be set.");
    let app_id = env::var("APIDECK_APP_ID").expect("APIDECK_APP_ID must be set.");

    let mut custom_headers = header::HeaderMap::new();
    custom_headers.insert(
        "Authorization",
        format!("Bearer {}", api_key).parse().unwrap(),
    );
    custom_headers.insert("x-apideck-app-id", app_id.parse().unwrap());
    custom_headers.insert("x-apideck-service-id", service.parse().unwrap());
    custom_headers.insert("x-apideck-consumer-id", consumer_id.parse().unwrap());

    return custom_headers;
}
