use crate::interfaces::cli::response::{CliResponse, CliStatus, Response};
use meval::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExprError {
    #[error("expr meval error {0}")]
    MevalError(Error),
}

impl Response for ExprError {
    fn into_response(&self) -> CliResponse {
        CliResponse::new(CliStatus::Error, Err(self.to_string()))
    }
}
