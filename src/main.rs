pub mod errors;
pub mod prism_crm;
pub mod service;
pub mod types;

use crate::prism_crm::{companies, opportunities, users};
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
            /*
             *          COMPANIES
             **/
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
            /*
             *          USERS
             **/
            .service(
                web::resource("/users/{user_id}").route(web::get().to(users::get_user::get_user)),
            )
            /*
             *          OPPORTUNITIES
             **/
            .service(
                web::resource("/opportunities")
                    .route(web::post().to(opportunities::post_opportunity::post_opportunity))
                    .route(web::get().to(opportunities::get_opportunities::get_opportunities)),
            )
            .service(
                web::resource("/opportunities/{company_id}")
                    .route(web::get().to(opportunities::get_opportunity::get_opportunity))
                    .route(web::patch().to(opportunities::update_opportunity::update_opportunity))
                    .route(web::delete().to(opportunities::delete_opportunity::delete_opportunity)),
            );

        App::new().service(scope)
    })
    .bind((env_config.host, env_config.port))?
    .run()
    .await
}
