use super::_types::{Lead, LeadFormatted, LeadFormattedList};
use crate::service::formatters::{
    format_money, get_language, get_primary_email, get_primary_phone,
};

impl Lead {
    fn monetary(&self) -> String {
        let currency = self.currency.as_deref().unwrap_or("USD");
        let monetary_amount = self.monetary_amount.unwrap_or(0);

        format_money(monetary_amount, currency)
    }

    fn get_name(&self) -> String {
        let first_name = self.first_name.clone().unwrap_or(String::from(""));
        let last_name = self.last_name.clone().unwrap_or(String::from(""));

        let name = format!("{} {}", first_name, last_name).trim().to_string();

        if name.len() > 0 {
            return name;
        }

        self.name.clone().unwrap_or(String::from(""))
    }

    pub fn format_list(&self) -> LeadFormattedList {
        let formatted = LeadFormattedList {
            id: self.id.clone(),
            name: self.get_name(),
            monetary_amount: self.monetary(),
            company_name: self.company_name.clone().unwrap_or(String::from("")),
            lead_source: self.lead_source.clone().unwrap_or(String::from("")),
            primary_email: get_primary_email(&self.emails),
            primary_phone: get_primary_phone(&self.phone_numbers),
        };

        formatted
    }

    pub fn format_one(&self) -> LeadFormatted {
        let language_code = &self.language.clone().unwrap_or(String::from("en"));

        let formatted = LeadFormatted {
            id: self.id.clone(),
            company_name: self.company_name.clone(),
            lead_source: self.lead_source.clone(),
            name: self.get_name(),
            description: self.description.clone(),
            language: get_language(language_code),
            status: self.status.clone(),
            monetary_amount: self.monetary(),
            websites: self.websites.clone(),
            addresses: self.addresses.clone(),
            social_links: self.social_links.clone(),
            phone_numbers: self
                .phone_numbers
                .clone()
                .unwrap_or(vec![])
                .into_iter()
                .map(|phone_number| phone_number.format())
                .collect(),
            emails: self.emails.clone(),
            custom_fields: self.custom_fields.clone(),
            tags: self.tags.clone(),

            company: None,
        };

        formatted
    }
}
