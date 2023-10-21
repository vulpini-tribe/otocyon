use actix_web::HttpResponse;

pub async fn post_company() -> HttpResponse {
    HttpResponse::Ok().body("OK".to_string())
}
