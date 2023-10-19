use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn get_companies() -> HttpResponse {
    HttpResponse::Ok().body("OK".to_string())
}
