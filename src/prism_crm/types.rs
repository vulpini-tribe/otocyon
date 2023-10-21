use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub id: Option<String>,
    pub _type: Option<String>, // 'primary' | 'secondary' | 'home' | 'office' | 'shipping' | 'billing' | 'other';
    pub string: Option<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub id: Option<String>,
    pub country_code: Option<String>,
    pub area_code: Option<String>,
    pub number: Option<String>,
    pub extension: Option<String>,
    pub _type: Option<String>,
}
