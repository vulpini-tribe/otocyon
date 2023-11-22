use super::_types::{Opportunity, OpportunityFormatted, OpportunityFormattedList};
use crate::companies::_types::Company;
use crate::contacts::_types::Contact;
use crate::leads::_types::Lead;
use crate::pipelines::_types::Pipeline;

use crate::service::formatters::format_money;

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

    pub fn format_list(&self, stage_name: String) -> OpportunityFormattedList {
        let formatted = OpportunityFormattedList {
            id: self.id.clone(),
            title: self.title.clone(),
            monetary_amount: self.monetary(),
            expected_revenue: self.revenue(),
            win_probability: self.win_probability.clone(),
            created_at: self.created_at.clone(),
            close_date: self.close_date.clone(),
            priority: self.priority.clone(),
            status: self.status.clone(),
            stage_name: Some(stage_name),
        };

        formatted
    }

    pub fn format_one(&self, external: (Company, Pipeline, Lead, Contact)) -> OpportunityFormatted {
        let (company, pipeline, lead, contact) = external;

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

            pipeline_id: self.pipeline_id.clone(),
            pipeline_stage_id: self.pipeline_stage_id.clone(),

            custom_fields: self.custom_fields.clone(),
            company: None,
            contact: None,
            pipeline: None,
            lead: None,
        };

        if self.company_id.is_some() {
            formatted.company = Some(company.format_one(None));
        }

        if self.primary_contact_id.clone().len() > 0 && self.primary_contact_id != "n/a" {
            formatted.contact = Some(contact);
        }

        if self.pipeline_id.is_some() {
            formatted.pipeline = Some(pipeline);
        }

        if self.lead_id.is_some() {
            formatted.lead = Some(lead);
        }

        formatted
    }
}
