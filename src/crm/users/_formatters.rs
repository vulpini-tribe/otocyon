use super::_types::{User, UserFormatted, UserFormattedList};
use crate::service::formatters::{get_primary_email, get_primary_phone};

impl User {
    pub fn format_list(&self) -> UserFormattedList {
        let primary_phone = get_primary_phone(&self.phone_numbers);
        let primary_email = get_primary_email(&self.emails);

        let formatted = UserFormattedList {
            id: self.id.clone(),
            name: format!(
                "{} {}",
                self.first_name.clone().unwrap_or(String::from("")),
                self.last_name.clone().unwrap_or(String::from("")),
            )
            .trim()
            .to_string(),
            company_name: self.company_name.clone().unwrap_or(String::from("")),
            image: self.image.clone().unwrap_or(String::from("")),
            status: self.status.clone().unwrap_or(String::from("")),
            primary_phone: Some(primary_phone),
            primary_email: Some(primary_email),
        };

        formatted
    }

    pub fn format_one(&self) -> UserFormatted {
        let formatted = UserFormatted {
            id: self.id.clone(),
        };

        formatted
    }
}
