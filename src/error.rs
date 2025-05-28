use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};
use std::fmt;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Wrong credentials error")]
    WrongCredentialsError,
    #[error("JWT creation error")]
    JWTTokenCreationError,
    #[error("JWT Token error")]
    JWTTokenError,
    #[error("Not authorized error")]
    NotAuthHeaderError,
    #[error("No permission error")]
    NoPermissionError,
    #[error("Invalid header name error")]
    InvalidHeaderName,
    #[error("Invalid Auth Header error")]
    InvalidAuthHeaderError,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
    status: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err:Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message): (StatusCode, String) = if err.is_not_found(){
        (StatusCode::NOT_FOUND, "Not found".to_string()) 
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::WrongCredentialsError => (StatusCode::FORBIDDEN, e.to_string()),
            Error::NoPermissionError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::JWTTokenError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::JWTTokenCreationError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),
            _ => (StatusCode::BAD_REQUEST, e.to_string())
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED, "Method not allowed".to_string()
        )
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string()
        )
    };

    let json = warp::reply::json(&ErrorResponse {
        status: code.to_string(),
        message
    });
    Ok(warp::reply::with_status(json, code))
}