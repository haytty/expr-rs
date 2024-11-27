use derive_more::Constructor;
use getset::Getters;
use serde::Serialize;

pub trait Response {
    fn into_response(&self) -> CliResponse;
}

#[derive(Debug, Constructor, Serialize, Getters)]
pub struct CliResponse {
    status: CliStatus,
    #[getset(get = "pub")]
    result: Result<f64, String>,
}

#[derive(Debug, Serialize)]
pub enum CliStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}
