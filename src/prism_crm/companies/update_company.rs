use actix_web::{web, HttpRequest, HttpResponse};

pub async fn update_company(_req: HttpRequest, _path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body("OK".to_string())
}
