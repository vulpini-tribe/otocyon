use super::contact_types::{Contact, ContactFormatted, ContactFormattedList};
use crate::types::Response;
use serde_json::Value;

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

pub fn to_contact(value: Response<Value>) -> Contact {
    let value = value.data.into();
    serde_json::from_value(value).unwrap()
}
