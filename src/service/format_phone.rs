use crate::types::PhoneNumber;

pub fn format_phone(phone_number: &PhoneNumber) -> Option<String> {
    let country_code = phone_number.country_code.as_deref().unwrap_or("");
    let area_code = phone_number.area_code.as_deref().unwrap_or("");
    let number = phone_number.number.as_deref().unwrap_or("");
    let ext = phone_number.extension.as_deref().unwrap_or("");

    let phone_number = format!("{} {} {}, {}", country_code, area_code, number, ext)
        .trim()
        .to_string();

    match phone_number.is_empty() {
        true => None,
        false => Some(phone_number),
    }
}
