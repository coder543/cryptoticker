use std;
use reqwest;
use serde_json;

#[derive(Debug)]
pub struct StrError(pub String);

impl std::convert::From<reqwest::Error> for StrError {
    fn from(error: reqwest::Error) -> Self {
        StrError(format!("{:#?}", error))
    }
}

impl std::convert::From<serde_json::Error> for StrError {
    fn from(error: serde_json::Error) -> Self {
        StrError(format!("{:#?}", error))
    }
}

impl std::convert::From<std::io::Error> for StrError {
    fn from(error: std::io::Error) -> Self {
        StrError(format!("{:#?}", error))
    }
}

impl std::convert::From<String> for StrError {
    fn from(error: String) -> Self {
        StrError(error)
    }
}
