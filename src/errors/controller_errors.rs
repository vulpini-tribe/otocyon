use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ServiceErrors {
    JsonError(serde_json::Error),
    JWTError(jsonwebtoken::errors::Error),
    RequestError(awc::error::SendRequestError),
    HTTPResponseError(awc::error::JsonPayloadError),
    SimpleTextException(String),
    InconsistentRole,
    ParsingError,
    MalformedRequest,
    AccessDenied,
    NoDbConnection,
    BadOriginHost,
    BadWSI,
    BadDashboardId,
    BadJWT,
    BadPayload,
    UnexpectedDBResponse,
    BadIntegrationToken,
}

impl fmt::Display for ServiceErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ServiceErrors::JsonError(ref err) => write!(f, "JSON error: {}", err),
            ServiceErrors::JWTError(ref err) => write!(f, "JWT validation error: {}", err),
            ServiceErrors::RequestError(ref err) => write!(f, "Request error: {}", err),
            ServiceErrors::HTTPResponseError(ref err) => {
                write!(f, "HTTP Response error in JSON parsing: {}", err)
            }
            ServiceErrors::SimpleTextException(ref err) => write!(f, "ICP error: {}", err),
            ServiceErrors::ParsingError => write!(f, "Malformed structure. Failed to parse."),
            ServiceErrors::MalformedRequest => write!(f, "Malformed Request."),
            ServiceErrors::AccessDenied => write!(f, "Access denied."),
            ServiceErrors::NoDbConnection => write!(f, "No DB connection."),
            ServiceErrors::BadOriginHost => write!(f, "Bad origin host."),
            ServiceErrors::BadWSI => write!(f, "Bad widget-store-id."),
            ServiceErrors::BadDashboardId => write!(f, "Bad dashboard id."),
            ServiceErrors::BadJWT => write!(f, "Bad JWT"),
            ServiceErrors::InconsistentRole => write!(f, "Requested role not in token role list."),
            ServiceErrors::BadPayload => write!(f, "Bad request payload."),
            ServiceErrors::UnexpectedDBResponse => write!(f, "Unexpected DB response."),
            ServiceErrors::BadIntegrationToken => write!(f, "Bad Integration token."),
        }
    }
}
impl Error for ServiceErrors {}

impl From<serde_json::Error> for ServiceErrors {
    fn from(err: serde_json::Error) -> ServiceErrors {
        ServiceErrors::JsonError(err)
    }
}

impl From<String> for ServiceErrors {
    fn from(err: String) -> ServiceErrors {
        ServiceErrors::SimpleTextException(err)
    }
}

impl From<jsonwebtoken::errors::Error> for ServiceErrors {
    fn from(err: jsonwebtoken::errors::Error) -> ServiceErrors {
        ServiceErrors::JWTError(err)
    }
}

impl From<awc::error::SendRequestError> for ServiceErrors {
    fn from(err: awc::error::SendRequestError) -> ServiceErrors {
        ServiceErrors::RequestError(err)
    }
}

impl From<awc::error::JsonPayloadError> for ServiceErrors {
    fn from(err: awc::error::JsonPayloadError) -> ServiceErrors {
        ServiceErrors::HTTPResponseError(err)
    }
}
