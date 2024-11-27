use crate::interfaces::cli::response::{CliResponse, CliStatus, Response};
use derive_more::Constructor;

#[derive(Debug, Constructor)]
pub struct ExprOutput {
    value: f64,
}

impl Response for ExprOutput {
    fn into_response(&self) -> CliResponse {
        CliResponse::new(CliStatus::Success, Ok(self.value.to_owned()))
    }
}
