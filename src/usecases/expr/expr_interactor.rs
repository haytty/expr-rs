use crate::usecases::expr::expr_error::ExprError;
use crate::usecases::expr::expr_input::ExprInput;
use crate::usecases::expr::expr_output::ExprOutput;
use crate::usecases::expr::expr_usecase::ExprUsecase;
use derive_more::Constructor;
use std::fmt::Debug;

#[derive(Debug, Constructor)]
pub struct ExprInteractor {}

impl ExprUsecase for ExprInteractor {
    fn expr(&self, input: ExprInput) -> Result<ExprOutput, ExprError> {
        let result = meval::eval_str(input.arg());

        match result {
            Ok(value) => Ok(ExprOutput::new(value)),
            Err(err) => Err(ExprError::MevalError(err)),
        }
    }
}
