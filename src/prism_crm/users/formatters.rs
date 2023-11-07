use super::user_types::{User, UserFormatted, UserFormattedList};

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
