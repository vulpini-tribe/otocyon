use crate::companies::get_company;
use crate::contacts::get_contact;
use crate::leads::get_lead;
use crate::pipelines::get_pipeline;
use crate::types::Response;

use actix_web::HttpRequest;
use serde_json::Value;

#[derive(Debug)]
pub enum RequestKinds {
    COMPANY,
    LEAD,
    CONTACT,
    PIPELINE,
}

pub async fn toss_request(
    req: &HttpRequest,
    entry_id: String,
    kind: RequestKinds,
) -> (Response<Value>, RequestKinds) {
    let response = match kind {
        RequestKinds::COMPANY => get_company::send_request(req, &entry_id).await,
        RequestKinds::LEAD => get_lead::send_request(req, &entry_id).await,
        RequestKinds::CONTACT => get_contact::send_request(req, &entry_id).await,
        RequestKinds::PIPELINE => get_pipeline::send_request(req, &entry_id).await,
    };

    (response, kind)
}
