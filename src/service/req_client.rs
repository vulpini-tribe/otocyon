use super::header_management::get_auth_headers;
use actix_web::HttpRequest;
use awc::Client;

pub fn req_client(req: &HttpRequest) -> Client {
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(&req.headers());

    let client = Client::builder()
        .add_default_header(("Authorization", auth))
        .add_default_header(("x-apideck-app-id", app_id))
        .add_default_header(("x-apideck-service-id", service_id))
        .add_default_header(("x-apideck-consumer-id", consumer_id))
        .finish();

    client
}
