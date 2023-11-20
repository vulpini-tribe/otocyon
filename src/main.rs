pub mod crm;
pub mod errors;
pub mod service;
pub mod types;
pub mod vault;

use crate::crm::{activities, companies, contacts, leads, notes, opportunities, pipelines, users};
use crate::vault::get_connections;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use log::info;
use service::service_logger::init_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    info!("Main bootstrap server starts");

    let env_config = service::env::EnvConfig::new();
    std::env::set_var("APIDECK_API_KEY", &env_config.api_key);
    std::env::set_var("APIDECK_APP_ID", &env_config.app_id);

    info!("[+] Setup ENV finished.");

    let redis = redis::Client::open(env_config.redis_url).unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .send_wildcard();

        let logger = Logger::default();

        App::new()
            .app_data(web::Data::new(redis.clone()))
            .wrap(logger)
            .wrap(cors)
            .service(
                web::scope("/prism")
                    .service(
                        web::resource("/vault")
                            .route(web::get().to(get_connections::get_connections)),
                    )
                    .service(
                        web::resource("/notes")
                            .route(web::post().to(notes::post_note::post_note))
                            .route(web::get().to(notes::get_notes::get_notes))
                            .route(web::delete().to(notes::delete_notes::delete_notes)),
                    )
                    .service(
                        web::resource("/notes/{note_id}")
                            .route(web::get().to(notes::get_note::get_note))
                            .route(web::patch().to(notes::update_note::update_note)),
                    )
                    .service(
                        web::resource("/activities")
                            .route(web::post().to(activities::post_activity::post_activity))
                            .route(web::get().to(activities::get_activities::get_activities))
                            .route(
                                web::delete().to(activities::delete_activities::delete_activities),
                            ),
                    )
                    .service(
                        web::resource("/activities/{activity_id}")
                            .route(web::get().to(activities::get_activity::get_activity))
                            .route(web::patch().to(activities::update_activity::update_activity)),
                    )
                    .service(
                        web::resource("/companies")
                            .route(web::post().to(companies::post_company::post_company))
                            .route(web::get().to(companies::get_companies::get_companies))
                            .route(web::delete().to(companies::delete_companies::delete_companies)),
                    )
                    .service(
                        web::resource("/companies/{company_id}")
                            .route(web::get().to(companies::get_company::get_company))
                            .route(web::patch().to(companies::update_company::update_company)),
                    )
                    .service(
                        web::resource("/users")
                            .route(web::post().to(users::post_user::post_user))
                            .route(web::get().to(users::get_users::get_users))
                            .route(web::delete().to(users::delete_users::delete_users)),
                    )
                    .service(
                        web::resource("/users/{user_id}")
                            .route(web::get().to(users::get_user::get_user))
                            .route(web::patch().to(users::update_user::update_user)),
                    )
                    .service(
                        web::resource("/opportunities")
                            .route(
                                web::post().to(opportunities::post_opportunity::post_opportunity),
                            )
                            .route(
                                web::get().to(opportunities::get_opportunities::get_opportunities),
                            )
                            .route(
                                web::delete()
                                    .to(opportunities::delete_opportunities::delete_opportunities),
                            ),
                    )
                    .service(
                        web::resource("/opportunities/{opportunity_id}")
                            .route(web::get().to(opportunities::get_opportunity::get_opportunity))
                            .route(
                                web::patch()
                                    .to(opportunities::update_opportunity::update_opportunity),
                            ),
                    )
                    .service(
                        web::resource("/contacts")
                            .route(web::post().to(contacts::post_contact::post_contact))
                            .route(web::get().to(contacts::get_contacts::get_contacts))
                            .route(web::delete().to(contacts::delete_contacts::delete_contacts)),
                    )
                    .service(
                        web::resource("/contacts/{contact_id}")
                            .route(web::get().to(contacts::get_contact::get_contact))
                            .route(web::patch().to(contacts::update_contact::update_contact)),
                    )
                    .service(
                        web::resource("/leads")
                            .route(web::post().to(leads::post_lead::post_lead))
                            .route(web::get().to(leads::get_leads::get_leads))
                            .route(web::delete().to(leads::delete_leads::delete_leads)),
                    )
                    .service(
                        web::resource("/leads/{lead_id}")
                            .route(web::get().to(leads::get_lead::get_lead))
                            .route(web::patch().to(leads::update_lead::update_lead)),
                    )
                    .service(
                        web::resource("/pipelines")
                            .route(web::post().to(pipelines::post_pipeline::post_pipeline))
                            .route(web::get().to(pipelines::get_pipelines::get_pipelines))
                            .route(web::delete().to(pipelines::delete_pipelines::delete_pipelines)),
                    )
                    .service(
                        web::resource("/pipelines/{pipeline_id}")
                            .route(web::get().to(pipelines::get_pipeline::get_pipeline))
                            .route(web::patch().to(pipelines::update_pipeline::update_pipeline)),
                    ),
            )
    })
    .bind((env_config.host, env_config.port))?
    .workers(2)
    .run()
    .await
}
