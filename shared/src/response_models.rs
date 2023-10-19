use domain::models::CompanyFormatted;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    #[serde(rename = "message")]
    Message(String),
    #[serde(rename = "company")]
    Company(CompanyFormatted),
    #[serde(rename = "companies")]
    Companies(Vec<CompanyFormatted>),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
