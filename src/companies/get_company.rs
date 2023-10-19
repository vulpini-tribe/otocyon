use actix_web::{get, HttpResponse};

#[get("/prism/company/{company_id}")]
pub async fn get_company(_company_id: String) -> HttpResponse {
    HttpResponse::Ok().body("OK".to_string())
}
