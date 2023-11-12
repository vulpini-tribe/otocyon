use super::_types::{Lead, LeadFormatted, LeadFormattedList};
use crate::service::formatters::{get_primary_email, get_primary_phone};

use crate::service::format_money::format_money;

impl Lead {
    fn monetary(&self) -> String {
        let currency = self.currency.as_deref().unwrap_or("USD");
        let monetary_amount = self.monetary_amount.unwrap_or(0);

        format_money(monetary_amount, currency)
    }

    pub fn format_list(&self) -> LeadFormattedList {
        let formatted = LeadFormattedList {
            id: self.id.clone(),
            name: self.name.clone().unwrap_or(String::from("")),
            monetary_amount: self.monetary(),
            company_name: self.company_name.clone().unwrap_or(String::from("")),
            lead_source: self.lead_source.clone().unwrap_or(String::from("")),
            primary_email: get_primary_email(&self.emails),
            primary_phone: get_primary_phone(&self.phone_numbers),
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
