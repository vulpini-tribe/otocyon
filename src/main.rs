pub mod errors;
pub mod prism_crm;
pub mod service;
pub mod types;

use crate::prism_crm::{companies, users};
use actix_web::{web, App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_config = service::env::EnvConfig::new();
    std::env::set_var("APIDECK_API_KEY", &env_config.api_key);
    std::env::set_var("APIDECK_APP_ID", &env_config.app_id);

    info!("[+] Setup ENV finished.");

    HttpServer::new(|| {
        let scope = web::scope("/prism")
            .service(
                web::resource("/companies")
                    .route(web::post().to(companies::post_company::post_company))
                    .route(web::get().to(companies::get_companies::get_companies)),
            )
            .service(
                web::resource("/companies/{company_id}")
                    .route(web::get().to(companies::get_company::get_company))
                    .route(web::patch().to(companies::update_company::update_company))
                    .route(web::delete().to(companies::delete_company::delete_company)),
            )
            .service(
                web::resource("/users/{user_id}").route(web::get().to(users::get_user::get_user)),
            );

        App::new().service(scope)
    })
    .bind((env_config.host, env_config.port))?
    .run()
    .await
}
