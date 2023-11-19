use crate::leads::_types::Lead;
use crate::leads::get_lead;
use crate::pipelines::_types::Pipeline;
use crate::pipelines::get_pipeline;
use crate::types::Response;
use crate::users::_types::User;
use crate::users::get_user;

use crate::companies::{_types::Company, get_company};
use crate::contacts::{_types::Contact, get_contact};
use crate::opportunities::{_types::Opportunity, get_opportunity};

use actix_web::{web, HttpRequest};

#[derive(Debug, PartialEq)]
pub enum RequestKinds {
    COMPANY,
    LEAD,
    CONTACT,
    PIPELINE,
    USER,
    OPPORTUNITY,
}

pub enum TossKindOr<C, L, P, CO, U, O> {
    Company(C),
    Lead(L),
    Pipeline(P),
    Contact(CO),
    User(U),
    Opportunity(O),
}

pub type TossKindOrType = TossKindOr<
    Response<Company>,
    Response<Lead>,
    Response<Pipeline>,
    Response<Contact>,
    Response<User>,
    Response<Opportunity>,
>;

impl TossKindOrType {
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

    pub fn user(self) -> Option<User> {
        match self {
            TossKindOr::User(user) => user.data,
            _ => None,
        }
    }

    pub fn opportunity(self) -> Option<Opportunity> {
        match self {
            TossKindOr::Opportunity(opportunity) => opportunity.data,
            _ => None,
        }
    }
}

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
        RequestKinds::USER => TossKindOr::User(get_user::send_request(req, &entry_id).await),
        RequestKinds::OPPORTUNITY => {
            TossKindOr::Opportunity(get_opportunity::send_request(req, &entry_id).await)
        }
    };

    return (request, kind);
}
