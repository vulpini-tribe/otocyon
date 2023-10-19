use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Deserialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct BankAccount {
    pub bank_name: String,
    pub account_number: String,
    pub account_name: String,
    pub account_type: String,
    pub iban: String,
    pub bic: String,
    pub routing_number: String,
    pub bsb_number: String,
    pub branch_identifier: String,
    pub bank_code: String,
    pub currency: String, // ISO 4217 currency code
}

#[derive(Deserialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Website {
    pub id: String,
    pub url: String,
    pub type_: String,
}

#[derive(Deserialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Address {
    pub id: String,
    pub type_: String, // 'primary' | 'secondary' | 'home' | 'office' | 'shipping' | 'billing' | 'other',
    pub string: String, // The address string
    pub name: String,
    pub line1: String,
    pub line2: String,
    pub line3: String,
    pub line4: String,
    pub street_number: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String, // ISO 3166-1 alpha-2 country code
    pub latitude: String,
    pub longitude: String,
    pub county: String,
    pub contact_name: String,
    pub salutation: String,
    pub phone_number: String,
    pub fax: String,
    pub email: String,
    pub website: String,
    pub notes: String,
    // A binary value used to detect updates to a object and prevent data conflicts.
    // It is incremented each time an update is made to the object.
    pub row_version: String,
}

#[derive(Deserialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct RowType {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct PhoneNumber {
    pub id: String,
    pub type_: String, // 'primary' | 'secondary' | 'home' | 'office' | 'mobile' | 'direct' | 'fax' | 'other',
    pub number: String,
    pub extension: String,
    pub country_code: String,
    pub area_code: String,
    pub notes: String,
    // A binary value used to detect updates to a object and prevent data conflicts.
    // It is incremented each time an update is made to the object.
    pub row_version: String,
}

#[derive(Deserialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Email {
    pub id: String,
    pub type_: String, // 'primary' | 'secondary' | 'home' | 'office' | 'other',
    pub email: String,
}

#[derive(Deserialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct CustomField {
    pub id: String,
    pub name: String,
    pub description: String,
    pub value: String,
}

#[derive(Deserialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Company {
    pub id: String,
    pub name: String,
    pub interaction_count: i32,
    pub owner_id: String,
    pub image: String,
    pub description: String,
    pub vat_number: String,
    pub currency: String,
    pub status: String,
    pub fax: String,
    pub annual_revenue: String,
    pub number_of_employees: String,
    pub industry: String,
    pub ownership: String,
    pub sales_tax_number: String,
    pub payee_number: String,
    pub abn_or_tfn: String,
    pub abn_branch: String,
    pub acn: String,
    pub first_name: String,
    pub last_name: String,
    pub parent_id: String,
    pub bank_accounts: Vec<BankAccount>,
    pub websites: Vec<Website>,
    pub addresses: Vec<Address>,
    pub social_links: Vec<Website>,
    pub phone_numbers: Vec<PhoneNumber>,
    pub emails: Vec<Email>,
    pub row_type: RowType,
    pub custom_fields: Vec<CustomField>,
    pub tags: Vec<String>,
    pub read_only: bool,
    pub last_activity_at: String,
    pub deleted: bool,
    pub salutation: String,
    pub birthday: String,
    pub updated_by: String,
    pub created_by: String,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CompanyFormatted {
    pub id: String,
    pub name: String,
}
