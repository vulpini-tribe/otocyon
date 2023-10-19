use actix_web::HttpResponse;

pub async fn update_company() -> HttpResponse {
    HttpResponse::Ok().body("OK".to_string())
}
