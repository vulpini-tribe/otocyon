use crate::prism_crm::users::_types;
use crate::types;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CompanyFormatted {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub contact_person: Option<String>, // salutation + first_name + last_name

    pub status: Option<String>,
    pub annual_revenue: Option<String>,
    pub number_of_employees: Option<String>,
    pub industry: Option<String>,
    pub ownership: Option<String>,

    pub tags: Option<Vec<String>>,

    pub bank_accounts: Option<Vec<types::BankAccount>>,
    pub websites: Option<Vec<types::Website>>,
    pub addresses: Option<Vec<types::Address>>,
    pub social_links: Option<Vec<types::Website>>,
    pub phone_numbers: Option<Vec<types::FormattedPhoneNumber>>,
    pub emails: Option<Vec<types::Email>>,

    pub owner: Option<_types::User>,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CompanyFormattedList {
    pub id: String,
    pub name: String,
    pub image: Option<String>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub annual_revenue: Option<String>,
    pub number_of_employees: Option<String>,
    pub industry: Option<String>,
    pub ownership: Option<String>,
    pub social_links: Option<Vec<types::Website>>,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: String,
    pub name: String, // Name of the company
    pub interaction_count: Option<i32>,
    pub owner_id: Option<String>,
    pub owner: Option<_types::User>,
    pub image: Option<String>,
    pub description: Option<String>,
    pub vat_number: Option<String>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub fax: Option<String>,
    pub annual_revenue: Option<String>,
    pub number_of_employees: Option<String>,
    pub industry: Option<String>,
    pub ownership: Option<String>,
    pub sales_tax_number: Option<String>,
    pub payee_number: Option<String>,
    pub abn_or_tfn: Option<String>,
    pub abn_branch: Option<String>,
    pub acn: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub parent_id: Option<String>,
    pub bank_accounts: Option<Vec<types::BankAccount>>,
    pub websites: Option<Vec<types::Website>>,
    pub addresses: Option<Vec<types::Address>>,
    pub social_links: Option<Vec<types::Website>>,
    pub phone_numbers: Option<Vec<types::PhoneNumber>>,
    pub emails: Option<Vec<types::Email>>,
    pub row_type: Option<types::RowType>,
    pub custom_fields: Option<Vec<types::CustomField>>,
    pub tags: Option<Vec<String>>,
    pub read_only: Option<bool>,
    pub last_activity_at: Option<String>,
    pub deleted: Option<bool>,
    pub salutation: Option<String>,
    pub birthday: Option<String>,
    pub updated_by: Option<String>,
    pub created_by: Option<String>,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PostCompany {
    pub name: String,
    pub image: Option<String>,
    pub description: Option<String>,
    pub vat_number: Option<String>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub fax: Option<String>,
    pub annual_revenue: Option<String>,
    pub number_of_employees: Option<String>,
    pub industry: Option<String>,
    pub ownership: Option<String>,
    pub sales_tax_number: Option<String>,
    pub payee_number: Option<String>,
    pub abn_or_tfn: Option<String>,
    pub abn_branch: Option<String>,
    pub acn: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bank_accounts: Option<Vec<types::BankAccount>>,
    pub websites: Option<Vec<types::Website>>,
    pub addresses: Option<Vec<types::Address>>,
    pub social_links: Option<Vec<types::Website>>,
    pub phone_numbers: Option<Vec<types::PhoneNumber>>,
    pub emails: Option<Vec<types::Email>>,
    pub row_type: Option<types::RowType>,
    pub custom_fields: Option<Vec<types::CustomField>>,
    pub tags: Option<Vec<String>>,
    pub read_only: Option<bool>,
    pub salutation: Option<String>,
    pub birthday: Option<String>,
}
