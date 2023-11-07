pub mod errors;
pub mod prism_crm;
pub mod service;
pub mod types;

use crate::prism_crm::{companies, contacts, leads, opportunities, pipelines, users};
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
                web::resource("/users")
                    .route(web::post().to(users::post_user::post_user))
                    .route(web::get().to(users::get_users::get_users)),
            )
            .service(
                web::resource("/users/{user_id}")
                    .route(web::get().to(users::get_user::get_user))
                    .route(web::patch().to(users::update_user::update_user))
                    .route(web::delete().to(users::delete_user::delete_user)),
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
                web::resource("/opportunities/{opportunity_id}")
                    .route(web::get().to(opportunities::get_opportunity::get_opportunity))
                    .route(web::patch().to(opportunities::update_opportunity::update_opportunity))
                    .route(web::delete().to(opportunities::delete_opportunity::delete_opportunity)),
            )
            /*
             *          CONTACTS
             **/
            .service(
                web::resource("/contacts")
                    .route(web::post().to(contacts::post_contact::post_contact))
                    .route(web::get().to(contacts::get_contacts::get_contacts)),
            )
            .service(
                web::resource("/contacts/{contact_id}")
                    .route(web::get().to(contacts::get_contact::get_contact))
                    .route(web::patch().to(contacts::update_contact::update_contact))
                    .route(web::delete().to(contacts::delete_contact::delete_contact)),
            )
            /*
             *          LEADS
             **/
            .service(
                web::resource("/leads")
                    .route(web::post().to(leads::post_lead::post_lead))
                    .route(web::get().to(leads::get_leads::get_leads)),
            )
            .service(
                web::resource("/leads/{lead_id}")
                    .route(web::get().to(leads::get_lead::get_lead))
                    .route(web::patch().to(leads::update_lead::update_lead))
                    .route(web::delete().to(leads::delete_lead::delete_lead)),
            )
            /*
             *          PIPELINES
             **/
            .service(
                web::resource("/pipelines")
                    .route(web::post().to(pipelines::post_pipeline::post_pipeline))
                    .route(web::get().to(pipelines::get_pipelines::get_pipelines)),
            )
            .service(
                web::resource("/pipelines/{pipeline_id}")
                    .route(web::get().to(pipelines::get_pipeline::get_pipeline))
                    .route(web::patch().to(pipelines::update_pipeline::update_pipeline))
                    .route(web::delete().to(pipelines::delete_pipeline::delete_pipeline)),
            );

        App::new().service(scope)
    })
    .bind((env_config.host, env_config.port))?
    .run()
    .await
}
