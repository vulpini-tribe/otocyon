use super::_types::{Lead, LeadFormatted, LeadFormattedList};

impl Lead {
    pub fn format_list(&self) -> LeadFormattedList {
        let formatted = LeadFormattedList {
            id: self.id.clone(),
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
