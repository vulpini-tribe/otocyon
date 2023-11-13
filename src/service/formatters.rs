use crate::types::{Email, PhoneNumber, Website};

pub fn get_primary_phone(phone_numbers: &Option<Vec<PhoneNumber>>) -> String {
    match phone_numbers {
        Some(phone_numbers) => {
            let primary_phone = phone_numbers
                .into_iter()
                .find(|phone_number| phone_number.r#type == Some("primary".to_string()));

            let primary_phone = match primary_phone {
                Some(phone_number) => Some(phone_number),
                None => phone_numbers.first(),
            };

            match primary_phone {
                Some(phone_number) => phone_number.collect_number().unwrap_or(String::from("")),
                None => return String::from(""),
            }
        }
        None => String::from(""),
    }
}

pub fn get_primary_email(emails: &Option<Vec<Email>>) -> String {
    match emails {
        Some(emails) => {
            let primary_email = emails
                .into_iter()
                .find(|email| email.r#type == Some("primary".to_string()));

            let primary_email = match primary_email {
                Some(email) => Some(email),
                None => emails.first(),
            };

            match primary_email {
                Some(email) => email.email.clone().unwrap_or(String::from("")),
                None => String::from(""),
            }
        }
        None => String::from(""),
    }
}

pub fn get_primary_website(websites: &Option<Vec<Website>>) -> String {
    match websites {
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