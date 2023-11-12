use super::_types::{Company, CompanyFormatted, CompanyFormattedList};
use crate::users::_types::User;

use crate::service::formatters::{get_primary_phone, get_primary_website};

impl Company {
    pub fn format_list(&self) -> CompanyFormattedList {
        let formatted = CompanyFormattedList {
            id: self.id.clone(),
            name: self.name.clone().unwrap_or(String::from("")),
            image: self.image.clone().unwrap_or(String::from("")),
            website: get_primary_website(&self.websites),
            primary_phone: get_primary_phone(&self.phone_numbers),
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
