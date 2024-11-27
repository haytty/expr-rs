use crate::containers::cli_service_container::CliServiceContainer;
use crate::interfaces::cli::requests::expr_request::ExprRequest;
use crate::interfaces::cli::response::{CliResponse, Response};
use crate::usecases::expr::expr_input::ExprInput;

pub fn handle(req: ExprRequest, cli_service_container: CliServiceContainer) -> CliResponse {
    let input: ExprInput = req.into();

    let result = cli_service_container.expr_usecase().expr(input);

    match result {
        Ok(o) => o.into_response(),
        Err(e) => e.into_response(),
    }
}
