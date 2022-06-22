use std::error::Error as StdError;

use hyper::StatusCode;
use serde::Deserialize;

#[derive(Debug)]
pub struct Error {
    pub(super) source: Option<Box<dyn StdError + Send + Sync>>,
    pub(super) r#type: ErrorType,
}

#[derive(Debug)]
pub enum ErrorType {
    ChunkingResponse,
    Parsing { body: Vec<u8> },
    RequestError,
    RequestTimeout,
    Response { body: Vec<u8>, status: StatusCode },
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}
