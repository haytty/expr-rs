use crate::usecases::expr::expr_input::ExprInput;
use derive_more::Constructor;
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Constructor, Validate, Deserialize)]
pub struct ExprRequest {
    #[validate(custom(function = "validate_numeric_and_operator"))]
    args: String,
}

const ALLOW_OPERATOR: &[char] = &['+', '-', '*', '/'];

fn validate_numeric_and_operator(args: &str) -> Result<(), ValidationError> {
    if args
        .chars()
        .all(|c| ALLOW_OPERATOR.contains(&c) || c.is_numeric())
    {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid Input"))
    }
}

impl Into<ExprInput> for ExprRequest {
    fn into(self) -> ExprInput {
        ExprInput::new(self.args)
    }
}
