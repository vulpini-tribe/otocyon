use crate::companies::get_company;
use crate::contacts::get_contact;
use crate::crm::companies::_types::Company;
use crate::crm::contacts::_types::Contact;
use crate::crm::leads::_types::Lead;
use crate::crm::pipelines::_types::Pipeline;
use crate::leads::get_lead;
use crate::pipelines::get_pipeline;
use crate::types::Response;

use actix_web::{web, HttpRequest};

#[derive(Debug, PartialEq)]
pub enum RequestKinds {
    COMPANY,
    LEAD,
    CONTACT,
    PIPELINE,
}

pub enum TossKindOr<S, T, F, U> {
    Company(S),
    Lead(T),
    Pipeline(F),
    Contact(U),
}

impl TossKindOr<Response<Company>, Response<Lead>, Response<Pipeline>, Response<Contact>> {
    pub fn company(self) -> Option<Company> {
        match self {
            TossKindOr::Company(company) => company.data,
            _ => None,
        }
    }

    pub fn lead(self) -> Option<Lead> {
        match self {
            TossKindOr::Lead(lead) => lead.data,
            _ => None,
        }
    }

    pub fn pipeline(self) -> Option<Pipeline> {
        match self {
            TossKindOr::Pipeline(pipeline) => pipeline.data,
            _ => None,
        }
    }

    pub fn contact(self) -> Option<Contact> {
        match self {
            TossKindOr::Contact(contact) => contact.data,
            _ => None,
        }
    }
}

pub type TossKindOrType =
    TossKindOr<Response<Company>, Response<Lead>, Response<Pipeline>, Response<Contact>>;

pub async fn toss_request(
    req: &HttpRequest,
    entry_id: String,
    kind: RequestKinds,
    redis: web::Data<redis::Client>,
) -> (TossKindOrType, RequestKinds) {
    let redis = redis.clone();

    let request = match kind {
        RequestKinds::COMPANY => {
            TossKindOr::Company(get_company::send_request(req, &entry_id).await)
        }
        RequestKinds::LEAD => TossKindOr::Lead(get_lead::send_request(req, &entry_id).await),
        RequestKinds::CONTACT => {
            TossKindOr::Contact(get_contact::send_request(req, &entry_id).await)
        }
        RequestKinds::PIPELINE => {
            TossKindOr::Pipeline(get_pipeline::send_request(req, &entry_id, redis).await)
        }
    };

    return (request, kind);
}
