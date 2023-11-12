use super::_types::{Contact, ContactFormatted, ContactFormattedList};
use crate::service::formatters::{get_primary_email, get_primary_phone, get_primary_website};

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
            primary_email: get_primary_email(&self.emails),
            primary_phone: get_primary_phone(&self.phone_numbers),
            website: get_primary_website(&self.websites),
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
