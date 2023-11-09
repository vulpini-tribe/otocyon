use crate::types;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Lead {
    pub id: String,
    pub name: Option<String>,
    pub currency: Option<String>,
    pub monetary_amount: Option<i64>,
    pub company_name: Option<String>,
    pub lead_source: Option<String>,
    pub emails: Option<Vec<types::Email>>,
    pub phone_numbers: Option<Vec<types::PhoneNumber>>,
    pub websites: Option<Vec<types::Website>>,
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
