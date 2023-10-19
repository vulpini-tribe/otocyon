use actix_web::{web, App, HttpServer};
mod companies;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/prism/company")
                    .route(web::delete().to(companies::delete_company::delete_company))
                    .route(web::patch().to(companies::update_company::update_company))
                    .route(web::post().to(companies::post_company::post_company)),
            )
            .service(companies::get_companies::get_companies)
            .service(companies::get_company::get_company)
    })
    .bind(("localhost", 1337))?
    .run()
    .await
}
