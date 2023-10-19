use actix_web::HttpResponse;

pub async fn delete_company() -> HttpResponse {
    HttpResponse::Ok().body("OK".to_string())
}
