use std::fmt::Debug;
use crate::usecases::expr::expr_error::ExprError;
use crate::usecases::expr::expr_input::ExprInput;
use crate::usecases::expr::expr_output::ExprOutput;

pub trait ExprUsecase: Debug {
    fn expr(&self, input: ExprInput) -> Result<ExprOutput, ExprError>;
}