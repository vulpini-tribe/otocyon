use crate::types::{Address, PhoneNumber};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CrmUser {
    pub id: String,
    pub parent_id: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub title: Option<String>,
    pub division: Option<String>,
    pub company_name: Option<String>,
    pub employee_number: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub language: Option<String>, // ISO 639-1
    pub status: Option<String>,
    pub password: Option<String>,
    pub addresses: Option<Vec<Address>>,
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
}
