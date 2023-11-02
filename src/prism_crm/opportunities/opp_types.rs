use crate::prism_crm::companies::company_types;
use crate::prism_crm::contacts::contact_types;
use crate::prism_crm::leads::lead_types;
use crate::prism_crm::pipelines::pipeline_types;
use serde_json::Value;

use crate::service::format_money::format_money;
use crate::types;
use serde::{Deserialize, Serialize};
use std::str;

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OpportunityFormattedList {
    pub title: String,
    pub id: Option<String>,
    pub r#type: Option<String>,
    pub monetary_amount: Option<String>, // monetary_amount + currency
    pub expected_revenue: Option<String>, // expected_revenue + currency
    pub win_probability: Option<i64>,
    pub close_date: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
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

    pub company: Option<company_types::Company>, // company: by company_id
    // pub contact: Option<contact_types::Contact>, // contact: by primary_contact_id
    // pub pipeline: Option<pipeline_types::Pipeline>, // pipeline: by pipeline_id + pipeline_stage_id
    // pub lead: Option<lead_types::Lead>,          // lead: by lead_id
    pub contact: Option<Value>,  // contact: by primary_contact_id
    pub pipeline: Option<Value>, // pipeline: by pipeline_id + pipeline_stage_id
    pub lead: Option<Value>,     // lead: by lead_id
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

impl Opportunity {
    fn monetary(&self) -> Option<String> {
        let currency = self.currency.as_deref().unwrap_or("USD");
        let monetary_amount = self.monetary_amount.unwrap_or(0);

        Some(format_money(monetary_amount, currency))
    }

    fn revenue(&self) -> Option<String> {
        let currency = self.currency.as_deref().unwrap_or("USD");
        let expected_revenue = self.expected_revenue.unwrap_or(0);

        Some(format_money(expected_revenue, currency))
    }

    pub fn format_list(&self) -> OpportunityFormattedList {
        let formatted = OpportunityFormattedList {
            id: self.id.clone(),
            title: self.title.clone(),
            r#type: self.r#type.clone(),
            monetary_amount: self.monetary(),
            expected_revenue: self.revenue(),
            win_probability: self.win_probability.clone(),
            close_date: self.close_date.clone(),
            priority: self.priority.clone(),
            status: self.status.clone(),
        };

        formatted
    }

    pub fn format(
        &self,
        external: (
            Option<company_types::Company>,
            Option<Value>,
            Option<Value>,
            Option<Value>,
            // Option<contact_types::Contact>,
            // Option<pipeline_types::Pipeline>,
            // Option<lead_types::Lead>,
        ),
    ) -> OpportunityFormatted {
        let (company, contact, pipeline, lead) = external;

        let mut formatted = OpportunityFormatted {
            id: self.id.clone(),
            title: self.title.clone(),
            description: self.description.clone(),
            r#type: self.r#type.clone(),
            monetary_amount: self.monetary(),
            expected_revenue: self.revenue(),
            win_probability: self.win_probability.clone(),
            close_date: self.close_date.clone(),
            loss_reason: self.loss_reason.clone(),
            won_reason: self.won_reason.clone(),
            priority: self.priority.clone(),
            status: self.status.clone(),
            tags: self.tags.clone(),
            company: None,
            contact: None,
            pipeline: None,
            lead: None,
        };

        if self.company_id.is_some() {
            // formatted.company = format_company(company);
            formatted.company = company;
        }

        if self.contact_id.is_some() {
            // formatted.contact = format_contact(contact);
            formatted.contact = contact;
        }

        if self.pipeline_id.is_some() {
            // formatted.pipeline = format_pipeline(pipeline);
            formatted.pipeline = pipeline;
        }

        if self.lead_id.is_some() {
            // formatted.lead = format_lead(lead);
            formatted.lead = lead;
        }

        formatted
    }
}
