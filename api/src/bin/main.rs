#[macro_use]
extern crate rocket;
use api::crm_handlers;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![crm_handlers::get_companies_handler,])
}

// Create cms connectors provider
// 1. Crete types for different resources (Company, Opportunity etc)
// 2. Create endpoint for request for CRUD operations, for each resource
// 3. Create request to apideck with current credentials for each CRUD and resource
// 3.5 Create another request, if needed, for example, for getting all opportunities for company
// 4. Parse and format response(s) from apideck and return it to user as json
// 5. Add redis cache
