use super::_types::{Contact, ContactFormatted, ContactFormattedList};
use crate::service::format_phone::format_phone;

impl Contact {
    fn get_image(&self) -> String {
        match &self.photo_url {
            Some(image) => image.clone(),
            None => self.image.clone().unwrap_or(String::from("")),
        }
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

    fn get_primary_website(&self) -> String {
        match &self.websites {
            Some(websites) => {
                let primary_website = websites
                    .into_iter()
                    .find(|website| website.r#type == Some("primary".to_string()));

                let primary_website = match primary_website {
                    Some(website) => Some(website),
                    None => websites.first(),
                };

                match primary_website {
                    Some(website) => website.clone().url.unwrap(),
                    None => String::from(""),
                }
            }
            None => String::from(""),
        }
    }

    pub fn format_list(&self) -> ContactFormattedList {
        let formatted = ContactFormattedList {
            id: self.id.clone(),
            image: self.get_image(),
            name: self.name.clone().unwrap_or(String::from("")),
            status: self.status.clone().unwrap_or(String::from("")),
            primary_email: self.get_primary_email(),
            primary_phone: self.get_primary_phone(),
            website: self.get_primary_website(),
            company_name: self.company_name.clone().unwrap_or(String::from("")),
        };

        formatted
    }

    pub fn format_one(&self) -> ContactFormatted {
        let formatted = ContactFormatted {
            id: self.id.clone(),
        };

        formatted
    }
}
