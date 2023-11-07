use super::_types::{Company, CompanyFormatted, CompanyFormattedList};
use crate::prism_crm::users::user_types::User;

impl Company {
    fn format_name(&self) -> Option<String> {
        let salutation = self.salutation.as_deref().unwrap_or("");
        let first_name = self.first_name.as_deref().unwrap_or("");
        let last_name = self.last_name.as_deref().unwrap_or("");

        let full_name = format!("{} {} {}", salutation, first_name, last_name)
            .trim()
            .to_string();

        match full_name.is_empty() {
            true => None,
            false => Some(full_name),
        }
    }

    pub fn format_list(&self) -> CompanyFormattedList {
        let formatted = CompanyFormattedList {
            id: self.id.clone(),
            name: self.name.clone(),
            image: self.image.clone(),
            currency: self.currency.clone(),
            status: self.status.clone(),
            annual_revenue: self.annual_revenue.clone(),
            number_of_employees: self.number_of_employees.clone(),
            industry: self.industry.clone(),
            ownership: self.ownership.clone(),
            social_links: self.social_links.clone(),
        };

        formatted
    }

    pub fn format_one(&self, owner: Option<User>) -> CompanyFormatted {
        let mut formatted = CompanyFormatted {
            id: self.id.clone(),
            name: self.name.clone(),

            description: self.description.clone(),
            contact_person: self.format_name(),
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
