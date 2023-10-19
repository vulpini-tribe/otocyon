use application::apideck_crm::companies_handler;
use domain::models::{Company, CompanyFormatted};
use rocket::get;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;

fn format_company(company: &Company) -> CompanyFormatted {
    return CompanyFormatted {
        id: company.id,
        name: company.name,
    };
}

#[get("/crm/companies")]
pub async fn get_companies_handler() -> Result<Json<Vec<CompanyFormatted>>, NotFound<String>> {
    let service = "hubspot".to_string();
    let consumer_id = "test-consumer".to_string();

    let companies = match companies_handler::companies_list(service, consumer_id).await {
        Ok(companies) => Ok(companies),
        Err(e) => Err(NotFound(e.to_string())),
    };

    Ok((companies.unwrap()).json().await.unwrap())
}
