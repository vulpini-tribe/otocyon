use crate::errors::controller_errors::ServiceErrors;
use actix_web::http::header::HeaderMap;
use std::ops::Add;

pub fn extract_header_as_result(
    headers: &HeaderMap,
    header_name: &str,
) -> Result<String, ServiceErrors> {
    headers
        .get(header_name)
        .map(|host| host.to_str().unwrap_or("").to_string())
        .ok_or(ServiceErrors::SimpleTextException(
            "Header not found: ".to_string().add(header_name),
        ))
}

pub fn get_headers(headers: &HeaderMap) -> (String, String, String, String) {
    let app_id = std::env::var("APIDECK_APP_ID");
    let auth = format!("Bearer {}", std::env::var("APIDECK_API_KEY").unwrap());
    let (consumer_id, service_id) = (
        extract_header_as_result(headers, "x-consumer-id"),
        extract_header_as_result(headers, "x-service-id"),
    );

    (
        app_id.unwrap(),
        auth,
        consumer_id.unwrap(),
        service_id.unwrap(),
    )
}
