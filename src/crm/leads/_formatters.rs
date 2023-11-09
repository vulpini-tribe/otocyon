use super::_types::{Lead, LeadFormatted, LeadFormattedList};

use crate::service::format_money::format_money;
use crate::service::format_phone::format_phone;

impl Lead {
    fn monetary(&self) -> String {
        let currency = self.currency.as_deref().unwrap_or("USD");
        let monetary_amount = self.monetary_amount.unwrap_or(0);

        format_money(monetary_amount, currency)
    }

    fn get_primary_email(&self) -> String {
        match &self.emails {
            Some(emails) => {
                let primary_email = emails
                    .into_iter()
                    .find(|email| email.r#type == Some("primary".to_string()));

                let primary_email = match primary_email {
                    Some(email) => Some(email),
                    None => emails.first(),
                };

                match primary_email {
                    Some(email) => email.email.clone().unwrap_or(String::from("")),
                    None => String::from(""),
                }
            }
            None => String::from(""),
        }
    }

    fn get_primary_phone(&self) -> String {
        match &self.phone_numbers {
            Some(phone_numbers) => {
                let primary_phone = phone_numbers
                    .into_iter()
                    .find(|phone_number| phone_number.r#type == Some("primary".to_string()));

                let primary_phone = match primary_phone {
                    Some(phone_number) => Some(phone_number),
                    None => phone_numbers.first(),
                };

                match primary_phone {
                    Some(phone_number) => format_phone(phone_number).unwrap_or(String::from("")),
                    None => return String::from(""),
                }
            }
            None => String::from(""),
        }
    }

    pub fn format_list(&self) -> LeadFormattedList {
        let formatted = LeadFormattedList {
            id: self.id.clone(),
            name: self.name.clone().unwrap_or(String::from("")),
            monetary_amount: self.monetary(),
            company_name: self.company_name.clone().unwrap_or(String::from("")),
            lead_source: self.lead_source.clone().unwrap_or(String::from("")),
            primary_email: self.get_primary_email(),
            primary_phone: self.get_primary_phone(),
        };

        formatted
    }

    pub fn format_one(&self) -> LeadFormatted {
        let formatted = LeadFormatted {
            id: self.id.clone(),
        };

        formatted
    }
}
