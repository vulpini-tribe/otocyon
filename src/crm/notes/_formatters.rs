use super::_types::{Note, NoteFormatted, NoteFormattedList};
use crate::companies::_types::Company;
use crate::contacts::_types::Contact;
use crate::leads::_types::Lead;
use crate::opportunities::_types::Opportunity;
use crate::users::_types::User;

impl Note {
    pub fn format_list(&self) -> NoteFormattedList {
        let formatted = NoteFormattedList {
            id: self.id.clone(),
            title: self.title.clone(),
            content: self.content.clone(),
            updated_at: self.updated_at.clone(),
            created_at: self.created_at.clone(),
        };

        formatted
    }

    pub fn format_one(
        &self,
        external: (
            Option<User>,
            Option<Company>,
            Option<Contact>,
            Option<Opportunity>,
            Option<Lead>,
        ),
    ) -> NoteFormatted {
        let (owner, company, contact, opportunity, lead) = external;

        let company_id = self.company_id.clone().unwrap_or(String::from(""));
        let contact_id = self.contact_id.clone().unwrap_or(String::from(""));
        let opportunity_id = self.opportunity_id.clone().unwrap_or(String::from(""));

        let mut formatted = NoteFormatted {
            id: self.id.clone(),
            title: self.title.clone().unwrap_or(String::from("")),
            content: self.content.clone().unwrap_or(String::from("")),
            active: self.active.clone().unwrap_or(true),

            owner: None,
            company: None,
            contact: None,
            opportunity: None,
            lead: None,
        };

        if self.owner_id.is_some() && owner.is_some() {
            formatted.owner = owner;
        }

        if company_id.len() > 0 && company_id != "n/a" && company.is_some() {
            formatted.company = company;
        }

        if contact_id.len() > 0 && contact_id != "n/a" && contact.is_some() {
            formatted.contact = contact;
        }

        if opportunity_id.len() > 0 && opportunity_id != "n/a" && opportunity.is_some() {
            formatted.opportunity = match opportunity {
                Some(opportunity) => Some(opportunity.format_one((None, None, None, None))),
                None => None,
            };
        }

        if self.lead_id.is_some() && lead.is_some() {
            formatted.lead = match lead {
                Some(lead) => Some(lead.format_one(None)),
                None => None,
            };
        }

        formatted
    }
}
