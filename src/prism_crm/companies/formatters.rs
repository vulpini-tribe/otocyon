use super::company_types::{Company, CompanyFormatted, CompanyFormattedForList};
use crate::prism_crm::users::user_types::CrmUser;
use crate::types::FormattedPhoneNumber;

pub fn format_name(
    salutation: &Option<String>,
    first_name: &Option<String>,
    last_name: &Option<String>,
) -> Option<String> {
    let salutation = salutation.clone().unwrap_or(String::from(""));
    let first_name = first_name.clone().unwrap_or(String::from(""));
    let last_name = last_name.clone().unwrap_or(String::from(""));

    let _name = format!("{} {} {}", salutation, first_name, last_name,)
        .trim()
        .to_string();

    match _name.is_empty() {
        true => None,
        false => Some(_name),
    }
}

pub fn format_phone_number(
    country_code: &Option<String>,
    area_code: &Option<String>,
    number: &Option<String>,
) -> Option<String> {
    let country_code = country_code.clone().unwrap_or(String::from(""));
    let area_code = area_code.clone().unwrap_or(String::from(""));
    let number = number.clone().unwrap_or(String::from(""));

    let _phone_number = format!("{} {} {}", country_code, area_code, number)
        .trim()
        .to_string();

    match _phone_number.is_empty() {
        true => None,
        false => Some(_phone_number),
    }
}

pub fn format_company_list(company: &Company) -> CompanyFormattedForList {
    let company = company;
    let formatted_company = CompanyFormattedForList {
        id: company.id.clone(),
        name: company.name.clone(),
        image: company.image.clone(),
        currency: company.currency.clone(),
        status: company.status.clone(),
        annual_revenue: company.annual_revenue.clone(),
        number_of_employees: company.number_of_employees.clone(),
        industry: company.industry.clone(),
        ownership: company.ownership.clone(),
        social_links: company.social_links.clone(),
    };

    formatted_company
}

pub fn format_company_extended(company: &Company, owner: Option<CrmUser>) -> CompanyFormatted {
    let company = company;
    let mut formatted_company = CompanyFormatted {
        id: company.id.clone(),
        name: company.name.clone(),

        description: company.description.clone(),
        contact_person: format_name(&company.salutation, &company.first_name, &company.last_name),
        status: company.status.clone(),
        annual_revenue: company.annual_revenue.clone(),
        number_of_employees: company.number_of_employees.clone(),
        industry: company.industry.clone(),
        ownership: company.ownership.clone(),

        tags: company.tags.clone(),

        bank_accounts: company.bank_accounts.clone(),
        websites: company.websites.clone(),
        addresses: company.addresses.clone(),
        social_links: company.social_links.clone(),
        phone_numbers: None,
        emails: company.emails.clone(),

        owner: None,
    };

    if owner.is_some() {
        let owner = owner.unwrap();
        formatted_company.owner = owner.clone().into();
    }

    if company.phone_numbers.is_some() {
        let phone_numbers = company.phone_numbers.clone().unwrap();
        let mut _phone_numbers: Vec<FormattedPhoneNumber> = vec![];

        phone_numbers.into_iter().for_each(|phone_number| {
            let phone_number = phone_number.clone();

            _phone_numbers.push(FormattedPhoneNumber {
                r#type: phone_number.r#type,
                number: format_phone_number(
                    &phone_number.country_code,
                    &phone_number.area_code,
                    &phone_number.number,
                ),
            });
        });

        formatted_company.phone_numbers = Some(_phone_numbers);
    }

    formatted_company
}
