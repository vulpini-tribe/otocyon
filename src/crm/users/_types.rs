use crate::types::{Address, Email, PhoneNumber};
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub parent_id: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub title: Option<String>,
    pub division: Option<String>,
    pub department: Option<String>,
    pub company_name: Option<String>,
    pub employee_number: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub language: Option<String>,
    pub status: Option<String>,
    pub password: Option<String>,
    pub addresses: Option<Vec<Address>>,
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    pub emails: Option<Vec<Email>>,
    pub updated_at: String,
    pub created_at: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct UserFormattedList {
    pub id: String,
    pub name: String,
    pub company_name: String,
    pub image: String,
    pub status: String,
    pub primary_phone: Option<String>,
    pub primary_email: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct UserFormatted {
    pub id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PostUser {
    pub title: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUsers {
    pub ids: Vec<String>,
}
