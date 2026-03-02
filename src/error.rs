// Error types
use std::error::Error as ErrorTrait;
use serde_json::Error as SerdeJSONError;
use chrono::format::ParseError as ChronoParseError;
use reqwest::Error as ReqwestError;
// Dependencies
use std::{fmt::Display, convert::From};


#[derive(Debug)]
pub enum Error {
    WrongResourceRequested,
    MissingQueryAttribute { attribute: String, },
    MissingKeyAttribute { attribute: String, },
    // Status code errors
    SC400,
    SC404,
    SC406,
    SC500,
    SC501,
    SC503,
    // Serde JSON errors
    SerdeJSONError(SerdeJSONError),
    // Chrono errors
    ChronoParseError(ChronoParseError),
    // Reqwest errors
    ReqwestError(ReqwestError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongResourceRequested => write!(f, "Wrong Resource Requested: The query's resource is incompatible with this operation."),
            Self::MissingQueryAttribute { attribute } => write!(f, "Missing Query Attribute: {attribute} is missing from the query."),
            Self::MissingKeyAttribute { attribute } => write!(f, "Missing Key Attribute: The key is missing a `{attribute}` attribute."),
            // Status code error
            Self::SC400 => write!(f, "Status Code 400 (Syntax Error): There is a syntactic or semantic issue with the parameters you supplied."),
            Self::SC404 => write!(f, "Status Code 404 (No Results Found): No results matching the query."),
            Self::SC406 => write!(f, "Status Code 406 (Not Acceptable): You ask for a resource representation that ECB Data Portal does not support."),
            Self::SC500 => write!(f, "Status Code 500 (Internal Server Error): ECB Data Portal internal issue."),
            Self::SC501 => write!(f, "Status Code 501 (Not Implemented): ECB Data Portal web service offers a subset of the functionality offered by the SDMX RESTful web service specification. You use a feature that has not yet been implemented."),
            Self::SC503 => write!(f, "Status Code 503 (Service Anavailable): ECB Data Portal service is unavailable."),
            // Serde JSON errors
            Self::SerdeJSONError(e) => write!(f, "Serde JSON Error: {}", e.to_string()),
            // Chrono errors
            Self::ChronoParseError(e) => write!(f, "Chrono Parse Error: {}", e.to_string()),
            // Reqwest errors
            Self::ReqwestError(e) => write!(f, "Reqwest Error: {}", e.to_string()),
        }
    }
}

impl ErrorTrait for Error {}

impl From<SerdeJSONError> for Error {
    fn from(value: SerdeJSONError) -> Self { Self::SerdeJSONError(value) }
}

impl From<ChronoParseError> for Error {
    fn from(value: ChronoParseError) -> Self { Self::ChronoParseError(value) }
}

impl From<ReqwestError> for Error {
    fn from(value: ReqwestError) -> Self { Self::ReqwestError(value) }
}