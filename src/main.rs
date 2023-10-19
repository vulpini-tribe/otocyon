use actix_web::{web, App, HttpServer};
use log::info;
mod companies;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_config = service::env::EnvConfig::new();
    std::env::set_var("APIDECK_API_KEY", &env_config.api_key);
    std::env::set_var("APIDECK_APP_ID", &env_config.app_id);

    info!("[+] Setup ENV finished.");

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/prism/company/{company_id}")
                    .route(web::get().to(companies::get_company::get_company))
                    .route(web::post().to(companies::post_company::post_company))
                    .route(web::patch().to(companies::update_company::update_company))
                    .route(web::delete().to(companies::delete_company::delete_company)),
            )
            .service(companies::get_companies::get_companies)
    })
    .bind(("localhost", 1337))?
    .run()
    .await
}
