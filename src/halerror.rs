use core::fmt;
use std::error;
use std::fmt::{Debug, Formatter};
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;


#[derive(Debug)]
pub enum HALError {
    GenericError(String),
    ReqwestError(ReqwestError),
    JsonError(JsonError)
}

impl fmt::Display for HALError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use HALError::*;
        match self {
            GenericError(err) => err.fmt(f),
            ReqwestError(err) => err.fmt(f),
            JsonError(err) => err.fmt(f)
        }
    }
}

impl error::Error for HALError {
    fn description(&self) -> &str {
        use HALError::*;
        match self {
            GenericError(_) => "Erreur générique",
            ReqwestError(_) => "Erreur HTTP API",
            JsonError(_) => "Erreur JSON"
        }
    }
}

impl From<ReqwestError> for HALError {
    fn from(err: ReqwestError) -> Self {
        Self::ReqwestError(err)
    }
}

impl From<JsonError> for HALError {
    fn from(err: JsonError) -> Self {
        Self::JsonError(err)
    }
}

impl From<String> for HALError {
    fn from(err: String) -> Self {
        Self::GenericError(err)
    }
}