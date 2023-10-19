use reqwest;
use shared::request_helpers;

pub async fn companies_list(
    service: String,
    consumer_id: String,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::builder().use_tokio().build()?;
    let custom_headers = request_helpers::populate_headers(service, consumer_id);

    let res = client
        .get("https://unify.apideck.com/crm/companies")
        .headers(custom_headers)
        .send()
        .await;

    return res;
}
