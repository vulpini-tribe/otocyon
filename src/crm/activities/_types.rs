// use crate::types;

use serde::{Deserialize, Serialize};
use std::str;

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
    pub archived: Option<bool>,
    pub deleted: Option<bool>,
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
pub struct DeleteOpportunities {
    pub ids: Vec<String>,
}
