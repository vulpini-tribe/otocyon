use serde::{Deserialize, Serialize};
use std::str;

// use crate::companies::_types::Company;
// use crate::contacts::_types::Contact;
// use crate::opportunities::_types::Opportunity;
// use crate::users::_types::User;

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct NoteFormattedList {
    pub id: String,
    pub title: String,
    pub content: String,
    pub active: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct NoteFormatted {
    pub id: String,
    pub title: String,
    pub content: String,
    pub active: bool,
    // pub owner: Option<User>,
    // pub company: Option<Company>,
    // pub contact: Option<Contact>,
    // pub opportunity: Option<Opportunity>,
    // pub lead: Option<Lead>,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub owner_id: String,
    pub company_id: Option<String>,
    pub contact_id: Option<String>,
    pub opportunity_id: Option<String>,
    pub lead_id: Option<String>,
    pub active: Option<bool>,
    pub updated_by: String,
    pub updated_at: String,
    pub created_by: String,
    pub created_at: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PostNote {
    pub title: String,
    pub primary_contact_id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DeleteNotes {
    pub ids: Vec<String>,
}
