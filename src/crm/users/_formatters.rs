use super::_types::{User, UserFormatted, UserFormattedList};
use crate::service::formatters::{get_language, get_primary_email, get_primary_phone};

impl User {
    fn get_name(&self) -> String {
        let first_name = self.first_name.clone().unwrap_or(String::from(""));
        let last_name = self.last_name.clone().unwrap_or(String::from(""));

        let name = format!("{} {}", first_name, last_name).trim().to_string();

        if name.len() > 0 {
            return name;
        }

        String::from("")
    }

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
        let language_code = &self.language.clone().unwrap_or(String::from("en"));

        let formatted = UserFormatted {
            id: self.id.clone(),
            name: self.get_name(),
            language: get_language(language_code),
            phone_numbers: self
                .phone_numbers
                .clone()
                .unwrap_or(vec![])
                .into_iter()
                .map(|phone_number| phone_number.format())
                .collect(),

            username: self.username.clone(),
            title: self.title.clone(),
            division: self.division.clone(),
            department: self.department.clone(),
            company_name: self.company_name.clone(),
            employee_number: self.employee_number.clone(),
            description: self.description.clone(),
            image: self.image.clone(),
            status: self.status.clone(),
            emails: self.emails.clone(),
        };

        formatted
    }
}
