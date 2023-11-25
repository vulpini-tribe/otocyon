use super::_types::{Contact, ContactFormatted, ContactFormattedList};
use crate::companies::_types::Company;
use crate::leads::_types::Lead;
use crate::service::formatters;

impl Contact {
    fn get_image(&self) -> String {
        match &self.photo_url {
            Some(image) => image.clone(),
            None => self.image.clone().unwrap_or(String::from("")),
        }
    }

    pub fn format_list(&self) -> ContactFormattedList {
        let formatted = ContactFormattedList {
            id: self.id.clone(),
            image: self.get_image(),
            name: self.name.clone().unwrap_or(String::from("")),
            status: self.status.clone().unwrap_or(String::from("")),
            primary_email: formatters::get_primary_email(&self.emails),
            primary_phone: formatters::get_primary_phone(&self.phone_numbers),
            website: formatters::get_primary_website(&self.websites),
            company_name: self.company_name.clone().unwrap_or(String::from("")),
        };

        formatted
    }

    pub fn format_one(&self, external: (Option<Company>, Option<Lead>)) -> ContactFormatted {
        let (company, lead) = external;

        let company_id = self.company_id.clone().unwrap_or(String::from(""));

        let full_name = formatters::get_full_name(
            self.first_name.as_deref().unwrap_or(""),
            self.middle_name.as_deref().unwrap_or(""),
            self.last_name.as_deref().unwrap_or(""),
        );

        let phone_numbers = self
            .phone_numbers
            .clone()
            .unwrap_or(vec![])
            .into_iter()
            .map(|phone_number| phone_number.format())
            .collect();

        let mut formatted = ContactFormatted {
            id: self.id.clone(),
            avatar: self.get_image(),
            name: full_name,
            r#type: self.r#type.clone(),
            status: self.status.clone(),
            emails: self.emails.clone(),
            phone_numbers: Some(phone_numbers),
            websites: self.websites.clone(),
            social_links: self.social_links.clone(),
            title: self.title.clone(),
            department: self.department.clone(),
            language: self.language.clone(),
            lead_source: self.lead_source.clone(),
            description: self.description.clone(),
            current_balance: self.current_balance.clone(),

            custom_fields: self.custom_fields.clone(),
            company: None,
            lead: None,
        };

        if company_id.len() > 0 && company_id != "n/a" && company.is_some() {
            formatted.company = company;
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
