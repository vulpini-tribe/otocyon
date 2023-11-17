use super::_types::{Contact, ContactFormatted, ContactFormattedList};
use crate::companies::_types::Company;
use crate::leads::_types::Lead;
use crate::service::formatters::{get_primary_email, get_primary_phone, get_primary_website};

impl Contact {
    fn get_image(&self) -> String {
        match &self.photo_url {
            Some(image) => image.clone(),
            None => self.image.clone().unwrap_or(String::from("")),
        }
    }

    fn get_full_name(&self) -> Option<String> {
        let first_name = self.first_name.clone().unwrap_or(String::from(""));
        let middle_name = self.middle_name.clone().unwrap_or(String::from(""));
        let last_name = self.last_name.clone().unwrap_or(String::from(""));

        let mut full_name = first_name.to_owned();

        if middle_name.len() > 0 {
            full_name.push_str(format!(" {middle_name}").as_str());
        }

        if last_name.len() > 0 {
            full_name.push_str(format!(" {last_name}").as_str());
        }

        full_name = full_name.trim().to_owned();

        match full_name.is_empty() {
            true => None,
            false => Some(full_name.to_owned()),
        }
    }

    pub fn format_list(&self) -> ContactFormattedList {
        let formatted = ContactFormattedList {
            id: self.id.clone(),
            image: self.get_image(),
            name: self.name.clone().unwrap_or(String::from("")),
            status: self.status.clone().unwrap_or(String::from("")),
            primary_email: get_primary_email(&self.emails),
            primary_phone: get_primary_phone(&self.phone_numbers),
            website: get_primary_website(&self.websites),
            company_name: self.company_name.clone().unwrap_or(String::from("")),
        };

        formatted
    }

    pub fn format_one(&self, external: (Company, Lead)) -> ContactFormatted {
        let (company, lead) = external;

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
            name: self.get_full_name(),
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

        if self.company_id.is_some() {
            formatted.company = Some(company.format_one(None));
        }

        if self.lead_id.is_some() {
            formatted.lead = Some(lead);
        }

        formatted
    }
}
