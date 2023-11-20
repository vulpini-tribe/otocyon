use crate::types;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Lead {
    pub id: String,
    pub name: Option<String>,
    pub company_name: Option<String>,
    pub owner_id: Option<String>,
    pub company_id: Option<String>,
    pub lead_id: Option<String>,
    pub lead_source: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub description: Option<String>,
    pub prefix: Option<String>,
    pub title: Option<String>,
    pub language: Option<String>,
    pub status: Option<String>,
    pub monetary_amount: Option<i64>,
    pub currency: Option<String>,
    pub fax: Option<String>,
    pub websites: Option<Vec<types::Website>>,
    pub addresses: Option<Vec<types::Address>>,
    pub social_links: Option<Vec<types::Website>>,
    pub phone_numbers: Option<Vec<types::PhoneNumber>>,
    pub emails: Option<Vec<types::Email>>,
    pub custom_fields: Option<Vec<types::CustomField>>,
    pub tags: Option<Vec<String>>,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LeadFormattedList {
    pub id: String,
    pub monetary_amount: String,
    pub company_name: String,
    pub lead_source: String,
    pub primary_email: String,
    pub primary_phone: String,
    pub name: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LeadFormatted {
    pub id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PostLead {
    pub title: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DeleteLeads {
    pub ids: Vec<String>,
}
