use super::company_types::PostCompany;
use crate::service::header_management::get_auth_headers;
// use crate::types::Response;
use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;
use serde_json::Value;

pub async fn send_request(req: &HttpRequest, _payload: PostCompany) -> Value {
    let client = Client::default();
    let (app_id, auth, consumer_id, service_id) = get_auth_headers(&req.headers());
    let payload = PostCompany {
        name: _payload.name,
        image: _payload.image,
        description: _payload.description,
        vat_number: _payload.vat_number,
        currency: _payload.currency,
        status: _payload.status,
        fax: _payload.fax,
        annual_revenue: _payload.annual_revenue,
        number_of_employees: _payload.number_of_employees,
        industry: _payload.industry,
        ownership: _payload.ownership,
        sales_tax_number: _payload.sales_tax_number,
        payee_number: _payload.payee_number,
        abn_or_tfn: _payload.abn_or_tfn,
        abn_branch: _payload.abn_branch,
        acn: _payload.acn,
        first_name: _payload.first_name,
        last_name: _payload.last_name,
        bank_accounts: match _payload.bank_accounts {
            None => Some(vec![]),
            Some(_) => _payload.bank_accounts,
        },
        websites: match _payload.websites {
            None => Some(vec![]),
            Some(_) => _payload.websites,
        },
        addresses: match _payload.addresses {
            None => Some(vec![]),
            Some(_) => _payload.addresses,
        },
        social_links: match _payload.social_links {
            None => Some(vec![]),
            Some(_) => _payload.social_links,
        },
        phone_numbers: match _payload.phone_numbers {
            None => Some(vec![]),
            Some(_) => _payload.phone_numbers,
        },
        emails: _payload.emails,
        row_type: _payload.row_type,
        custom_fields: match _payload.custom_fields {
            None => Some(vec![]),
            Some(_) => _payload.custom_fields,
        },
        tags: match _payload.tags {
            None => Some(vec![]),
            Some(_) => _payload.tags,
        },
        read_only: _payload.read_only,
        salutation: _payload.salutation,
        birthday: _payload.birthday,
        ..Default::default()
    };

    let payload = serde_json::json!(payload);

    let response = client
        .post("https://unify.apideck.com/crm/companies")
        .insert_header(("Authorization", auth))
        .insert_header(("x-apideck-app-id", app_id))
        .insert_header(("x-apideck-service-id", service_id))
        .insert_header(("x-apideck-consumer-id", consumer_id))
        .send_json(&payload)
        .await;

    return response.unwrap().json::<Value>().await.unwrap();
}

pub async fn post_company(req: HttpRequest, payload: web::Json<PostCompany>) -> HttpResponse {
    let main_request = send_request(&req, payload.clone()).await;

    HttpResponse::Ok().json(web::Json(main_request))
}
