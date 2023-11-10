use super::_types::{Company, CompanyFormatted, CompanyFormattedList};
use crate::service::format_phone::format_phone;
use crate::users::_types::User;

impl Company {
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

    pub fn format_list(&self) -> CompanyFormattedList {
        let formatted = CompanyFormattedList {
            id: self.id.clone(),
            name: self.name.clone(),
            image: self.image.clone().unwrap_or(String::from("")),
            website: self.get_primary_website(),
            primary_phone: self.get_primary_phone(),
            created_at: self.created_at.clone().unwrap_or(String::from("")),
        };

        formatted
    }

    pub fn format_one(&self, owner: Option<User>) -> CompanyFormatted {
        let mut formatted = CompanyFormatted {
            id: self.id.clone(),
            name: self.name.clone(),

            description: self.description.clone(),
            contact_person: self.first_name.clone(),
            status: self.status.clone(),
            annual_revenue: self.annual_revenue.clone(),
            number_of_employees: self.number_of_employees.clone(),
            industry: self.industry.clone(),
            ownership: self.ownership.clone(),

            tags: self.tags.clone(),

            bank_accounts: self.bank_accounts.clone(),
            websites: self.websites.clone(),
            addresses: self.addresses.clone(),
            social_links: self.social_links.clone(),
            phone_numbers: None,
            emails: self.emails.clone(),

            owner: None,
        };

        if owner.is_some() {
            let owner = owner.unwrap();
            formatted.owner = owner.clone().into();
        }

        if self.phone_numbers.is_some() {
            let phone_numbers = self
                .phone_numbers
                .clone()
                .unwrap()
                .into_iter()
                .map(|phone_number| phone_number.format())
                .collect();

            formatted.phone_numbers = Some(phone_numbers);
        }

        formatted
    }
}
