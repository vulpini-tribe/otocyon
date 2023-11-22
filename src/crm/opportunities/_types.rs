use crate::companies::_types::CompanyFormatted;
use crate::contacts::_types::Contact;
use crate::leads::_types::Lead;
use crate::pipelines::_types::Pipeline;
use crate::types;

use serde::{Deserialize, Serialize};
use std::str;

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OpportunityFormattedList {
    pub title: String,
    pub id: Option<String>,
    pub monetary_amount: Option<String>, // monetary_amount + currency
    pub expected_revenue: Option<String>, // expected_revenue + currency
    pub win_probability: Option<i64>,
    pub close_date: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub stage_name: Option<String>, // {pipeline_id}.{pipeline_stage_id}
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OpportunityFormatted {
    pub title: String,
    pub description: Option<String>,
    pub id: Option<String>,
    pub r#type: Option<String>,
    pub monetary_amount: Option<String>, // monetary_amount + currency
    pub expected_revenue: Option<String>, // expected_revenue + currency
    pub win_probability: Option<i64>,
    pub close_date: Option<String>,
    pub loss_reason: Option<String>,
    pub won_reason: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub tags: Option<Vec<String>>,
    pub pipeline_id: Option<String>,
    pub pipeline_stage_id: Option<String>,

    pub custom_fields: Option<Vec<types::CustomField>>,
    pub company: Option<CompanyFormatted>, // company: by company_id
    pub contact: Option<Contact>,          // contact: by primary_contact_id
    pub pipeline: Option<Pipeline>,        // pipeline: by pipeline_id + pipeline_stage_id
    pub lead: Option<Lead>,                // lead: by lead_id
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Opportunity {
    pub title: String,
    pub primary_contact_id: String,
    pub id: Option<String>,
    pub description: Option<String>,
    pub r#type: Option<String>,
    pub monetary_amount: Option<i64>,
    pub currency: Option<String>,
    pub win_probability: Option<i64>,
    pub expected_revenue: Option<i64>,
    pub close_date: Option<String>,
    pub loss_reason_id: Option<String>,
    pub loss_reason: Option<String>,
    pub won_reason_id: Option<String>,
    pub won_reason: Option<String>,
    pub pipeline_id: Option<String>,
    pub pipeline_stage_id: Option<String>,
    pub source_id: Option<String>,
    pub lead_id: Option<String>,
    pub lead_source: Option<String>,
    pub contact_id: Option<String>,
    pub contact_ids: Option<Vec<String>>,
    pub company_id: Option<String>,
    pub company_name: Option<String>,
    pub owner_id: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub status_id: Option<String>,
    pub tags: Option<Vec<String>>,
    pub interaction_count: Option<i64>,
    pub custom_fields: Option<Vec<types::CustomField>>,
    pub stage_last_changed_at: Option<String>,
    pub last_activity_at: Option<String>,
    pub deleted: Option<bool>,
    pub date_stage_changed: Option<String>,
    pub date_last_contacted: Option<String>,
    pub date_lead_created: Option<String>,
    pub updated_by: Option<String>,
    pub created_by: Option<String>,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PostOpportunity {
    pub title: String,
    pub primary_contact_id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOpportunities {
    pub ids: Vec<String>,
}
