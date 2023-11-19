use crate::types;
use serde::{Deserialize, Serialize};
use std::str;

use crate::crm::{
    companies::_types::Company, contacts::_types::Contact, opportunities::_types::Opportunity,
    users::_types::User,
};

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ActivityFormattedList {
    pub id: String,
    pub r#type: String,
    pub title: String,
    pub updated_at: String,
    pub description: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ActivityFormatted {
    pub id: String,
    pub r#type: String,
    pub title: String,
    pub updated_at: String,
    pub description: Option<String>,
    pub duration: Option<i64>,
    pub is_all_day: Option<bool>,
    pub private: Option<bool>,
    pub recurrent: Option<bool>,
    pub reminder_set: Option<bool>,
    pub reminder_datetime: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub video_conference_url: Option<String>,
    pub video_conference_id: Option<String>,
    pub attendees: Option<Vec<types::AttendeeFormatted>>,
    pub custom_fields: Option<Vec<types::CustomField>>,

    pub owner: Option<User>,              // owner_id
    pub company: Option<Company>,         // company_id
    pub contact: Option<Contact>,         // contact_id
    pub opportunity: Option<Opportunity>, // opportunity_id
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub r#type: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub owner_id: Option<String>,
    pub account_id: Option<String>,
    pub company_id: Option<String>,
    pub contact_id: Option<String>,
    pub opportunity_id: Option<String>,
    pub duration_seconds: Option<i64>,
    pub activity_datetime: Option<String>,
    pub all_day_event: Option<bool>,
    pub start_datetime: Option<String>,
    pub end_datetime: Option<String>,
    pub private: Option<bool>,
    pub recurrent: Option<bool>,
    pub reminder_set: Option<bool>,
    pub reminder_datetime: Option<String>,
    pub archived: Option<bool>,
    pub deleted: Option<bool>,
    pub attendees: Option<Vec<types::Attendee>>,
    pub video_conference_url: Option<String>,
    pub video_conference_id: Option<String>,
    pub custom_fields: Option<Vec<types::CustomField>>,
    pub updated_by: String,
    pub updated_at: String,
    pub created_by: String,
    pub created_at: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PostActivity {
    pub title: String,
    pub primary_contact_id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DeleteActivities {
    pub ids: Vec<String>,
}
