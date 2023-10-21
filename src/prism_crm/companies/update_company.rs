use actix_web::{web, HttpRequest, HttpResponse};

pub async fn update_company(_req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let company_id = path.into_inner();
    let app_id = std::env::var("APIDECK_APP_ID").unwrap();
    let api_key = std::env::var("APIDECK_API_KEY").unwrap();

    HttpResponse::Ok().body(format!(
        "ID: {}\nAPP_ID: {}\nAPI_KEY: {}\n",
        company_id, app_id, api_key,
    ))
}
