use super::opp_types::{Opportunity, OpportunityFormatted, OpportunityFormattedList};
use crate::prism_crm::companies::company_types::Company;
use crate::prism_crm::contacts::contact_types::Contact;
use crate::prism_crm::leads::lead_types::Lead;
use crate::prism_crm::pipelines::pipeline_types::Pipeline;
use crate::types::Response;
use serde_json::Value;

use crate::service::format_money::format_money;

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
            company: None,
            contact: None,
            pipeline: None,
            lead: None,
        };

        if self.company_id.is_some() {
            // formatted.company = format_company(company);
            formatted.company = Some(company);
        }

        if self.contact_id.is_some() {
            // formatted.contact = format_contact(contact);
            formatted.contact = Some(contact);
        }

        if self.pipeline_id.is_some() {
            // formatted.pipeline = format_pipeline(pipeline);
            formatted.pipeline = Some(pipeline);
        }

        if self.lead_id.is_some() {
            // formatted.lead = format_lead(lead);
            formatted.lead = Some(lead);
        }

        formatted
    }
}

pub fn to_opportunity(value: Response<Value>) -> Opportunity {
    let value = value.data.into();
    serde_json::from_value(value).unwrap()
}
