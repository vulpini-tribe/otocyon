use crate::types::{Email, PhoneNumber, Website};
use num_format::{Locale, ToFormattedString};
use rusty_money::{iso, Money};

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
                None => String::from(""),
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

pub fn get_full_name(first_name: &str, middle_name: &str, last_name: &str) -> Option<String> {
    let mut full_name = first_name.to_owned();

    if middle_name.len() > 0 {
        full_name.push_str(format!(" {middle_name}").as_str());
    }

    if last_name.len() > 0 {
        full_name.push_str(format!(" {last_name}").as_str());
    }

    full_name = full_name.trim().to_owned();

    match full_name.is_empty() {
        true => None,
        false => Some(full_name.to_owned()),
    }
}

pub fn format_money(monetary_amount: i64, currency: &str) -> String {
    let currency = iso::find(currency).unwrap_or(iso::USD);
    let money = Money::from_major(monetary_amount, currency);
    let currency = money.currency();
    let amount = monetary_amount.to_formatted_string(&Locale::en);

    match currency.symbol_first {
        true => format!("{}\u{2009}{}", currency.symbol, amount),
        false => format!("{}\u{2009}{}", amount, currency.symbol),
    }
}
