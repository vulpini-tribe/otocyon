use crate::companies::_types::Company;
use crate::leads::_types::Lead;
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
    pub social_links: Option<Vec<types::Website>>,
    pub active: Option<bool>,
    pub owner_id: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub gender: Option<String>,
    pub birthday: Option<String>,
    pub fax: Option<String>,
    pub addresses: Option<Vec<types::Address>>,
    pub email_domain: Option<String>,
    pub tags: Option<Vec<String>>,
    pub company_name: Option<String>,
    pub custom_fields: Option<Vec<types::CustomField>>,
    pub company_id: Option<String>,
    pub lead_id: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub title: Option<String>,
    pub department: Option<String>,
    pub language: Option<String>,
    pub lead_source: Option<String>,
    pub description: Option<String>,
    pub current_balance: Option<i64>,

    pub first_call_at: Option<String>,
    pub first_email_at: Option<String>,
    pub last_activity_at: Option<String>,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ContactFormatted {
    pub id: String,
    pub avatar: String,
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub status: Option<String>,
    pub emails: Option<Vec<types::Email>>,
    pub phone_numbers: Option<Vec<types::FormattedPhoneNumber>>,
    pub websites: Option<Vec<types::Website>>,
    pub social_links: Option<Vec<types::Website>>,
    pub title: Option<String>,
    pub department: Option<String>,
    pub language: Option<String>,
    pub lead_source: Option<String>,
    pub description: Option<String>,
    pub current_balance: Option<i64>,

    pub custom_fields: Option<Vec<types::CustomField>>,
    pub company: Option<Company>,
    pub lead: Option<Lead>,
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
