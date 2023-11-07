use super::user_types::{User, UserFormatted, UserFormattedList};
use crate::types::Response;
use serde_json::Value;

impl User {
    pub fn format_list(&self) -> UserFormattedList {
        let formatted = UserFormattedList {
            id: self.id.clone(),
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

pub fn to_user(value: Response<Value>) -> User {
    let value = value.data.into();
    serde_json::from_value(value).unwrap()
}
