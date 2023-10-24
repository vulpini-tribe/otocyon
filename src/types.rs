use serde::{Deserialize, Serialize};

// Network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePayload {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub status_code: Option<u16>,
    pub status: Option<String>,
}

// Entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub bank_name: Option<String>,
    pub account_number: Option<String>,
    pub account_name: Option<String>,
    pub account_type: Option<String>,
    pub iban: Option<String>,
    pub bic: Option<String>,
    pub routing_number: Option<String>,
    pub bsb_number: Option<String>,
    pub branch_identifier: Option<String>,
    pub bank_code: Option<String>,
    pub currency: Option<String>, // ISO 4217 currency code
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Website {
    pub id: Option<String>,
    pub url: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub id: Option<String>,
    pub type_: Option<String>, // 'primary' | 'secondary' | 'home' | 'office' | 'shipping' | 'billing' | 'other',
    pub string: Option<String>, // The address string
    pub name: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub line3: Option<String>,
    pub line4: Option<String>,
    pub street_number: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>, // ISO 3166-1 alpha-2 country code
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub county: Option<String>,
    pub contact_name: Option<String>,
    pub salutation: Option<String>,
    pub phone_number: Option<String>,
    pub fax: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub notes: Option<String>,
    // A binary value used to detect updates to a object and prevent data conflicts.
    // It is incremented each time an update is made to the object.
    pub row_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub id: Option<String>,
    pub country_code: Option<String>,
    pub area_code: Option<String>,
    pub number: Option<String>,
    pub extension: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattedPhoneNumber {
    pub number: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub id: String,
    pub type_: String, // 'primary' | 'secondary' | 'home' | 'office' | 'other',
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowType {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomField {
    pub id: String,
    pub name: String,
    pub description: String,
    pub value: String,
}
