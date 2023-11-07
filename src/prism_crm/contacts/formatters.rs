use super::contact_types::{Contact, ContactFormatted, ContactFormattedList};

impl Contact {
    pub fn format_list(&self) -> ContactFormattedList {
        let formatted = ContactFormattedList {
            id: self.id.clone(),
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
