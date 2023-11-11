use crate::companies::get_company;
use crate::contacts::get_contact;
use crate::crm::companies::_types::Company;
use crate::crm::contacts::_types::Contact;
use crate::crm::leads::_types::Lead;
use crate::crm::pipelines::_types::Pipeline;
use crate::leads::get_lead;
use crate::pipelines::get_pipeline;
use crate::types::Response;

use actix_web::HttpRequest;
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub enum RequestKinds {
    COMPANY,
    LEAD,
    CONTACT,
    PIPELINE,
}

pub enum TypeOr<S, T, F, U> {
    Left(S),
    Right(T),
    One(F),
    Two(U),
}

pub async fn toss_request(
    req: &HttpRequest,
    entry_id: String,
    kind: RequestKinds,
) -> (
    TypeOr<Response<Company>, Response<Lead>, Response<Pipeline>, Response<Contact>>,
    RequestKinds,
) {
    if kind == RequestKinds::COMPANY {
        let response: Response<Company> = get_company::send_request(req, &entry_id).await;
        return (TypeOr::Left(response), kind);
    } else if kind == RequestKinds::LEAD {
        let response: Response<Lead> = get_lead::send_request(req, &entry_id).await;
        return (TypeOr::Right(response), kind);
    } else if kind == RequestKinds::CONTACT {
        let response: Response<Contact> = get_contact::send_request(req, &entry_id).await;
        return (TypeOr::Two(response), kind);
    } else if kind == RequestKinds::PIPELINE {
        let response: Response<Pipeline> = get_pipeline::send_request(req, &entry_id).await;
        return (TypeOr::One(response), kind);
    } else {
        panic!("Invalid RequestKinds");
    }
}
