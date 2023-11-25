use crate::crm::{
    companies::_types::Company, contacts::_types::Contact, opportunities::_types::Opportunity,
    users::_types::User,
};

use super::_types::{Activity, ActivityFormatted, ActivityFormattedList};

impl Activity {
    pub fn duration(&self) -> Option<i64> {
        // @todo: convert to HH:mm
        self.duration_seconds.clone()
    }

    pub fn format_list(&self) -> ActivityFormattedList {
        let formatted = ActivityFormattedList {
            id: self.id.clone(),
            r#type: self.r#type.clone(),
            title: self.title.clone().unwrap_or(String::from("")),
            updated_at: self.updated_at.clone(),
            description: self.description.clone(),
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
        ),
    ) -> ActivityFormatted {
        let (user, company, contact, opportunity) = external;

        let company_id = self.company_id.clone().unwrap_or(String::from(""));
        let contact_id = self.contact_id.clone().unwrap_or(String::from(""));
        let opportunity_id = self.opportunity_id.clone().unwrap_or(String::from(""));

        let attendees = self
            .attendees
            .clone()
            .unwrap_or(vec![])
            .into_iter()
            .map(|attendee| attendee.format())
            .collect();

        let mut formatted = ActivityFormatted {
            id: self.id.clone(),
            r#type: self.r#type.clone(),
            title: self.title.clone().unwrap_or(String::from("")),
            updated_at: self.updated_at.clone(),
            description: self.description.clone(),
            duration: self.duration(),
            is_all_day: self.all_day_event.clone(),
            start_time: self.activity_datetime.clone(),
            end_time: self.end_datetime.clone(),
            attendees: Some(attendees),
            recurrent: self.recurrent.clone(),
            reminder_set: self.reminder_set.clone(),
            reminder_datetime: self.reminder_datetime.clone(),
            private: self.private.clone(),
            video_conference_url: self.video_conference_url.clone(),
            video_conference_id: self.video_conference_id.clone(),
            custom_fields: self.custom_fields.clone(),

            owner: None,       // owner_id
            company: None,     // company_id
            contact: None,     // contact_id
            opportunity: None, // opportunity_id
        };

        if self.owner_id.is_some() {
            formatted.owner = user;
        }

        if company_id.len() > 0 && company_id != "n/a" && company.is_some() {
            formatted.company = company;
        }

        if contact_id.len() > 0 && contact_id != "n/a" {
            formatted.contact = contact;
        }

        if opportunity_id.len() > 0 && opportunity_id != "n/a" && opportunity.is_some() {
            formatted.opportunity = match opportunity {
                Some(opportunity) => Some(opportunity.format_one((None, None, None, None))),
                None => None,
            };
        }
        formatted
    }
}
