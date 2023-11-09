use crate::types;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub photo_url: Option<String>,
    pub image: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub status: Option<String>,
    pub emails: Option<Vec<types::Email>>,
    pub phone_numbers: Option<Vec<types::PhoneNumber>>,
    pub websites: Option<Vec<types::Website>>,
    pub active: Option<bool>,
    pub company_name: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ContactFormatted {
    pub id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ContactFormattedList {
    pub id: String,
    pub image: String,
    pub name: String,
    pub status: String,
    pub primary_email: String,
    pub primary_phone: String,
    pub website: String,
    pub company_name: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PostContact {
    pub title: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DeleteContacts {
    pub ids: Vec<String>,
}
