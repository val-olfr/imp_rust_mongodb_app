use mongodb::bson;
use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, reply, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("Error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),

    #[error("Could not access field in document: {0}")]
    MongoDataError(#[from] bson::document::ValueAccessError),
    
    #[error("Invalid id used: {0}")]
    InvalidIDError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<Box<dyn Reply>, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<Error>() {
        match e {
            _ => {
                eprintln!("Unhadled application error: {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error (application)";
            }
        }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        eprintln!("Unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error (general)"
    }

    let json = reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(Box::new(reply::with_status(json, code)))
}