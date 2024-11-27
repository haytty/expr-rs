use crate::containers::cli_service_container::CliServiceContainer;
use crate::interfaces::cli;
use crate::interfaces::cli::requests::expr_request::ExprRequest;
use anyhow::Result;
use clap::{Args, ValueHint};
use validator::Validate;

/// Structure to hold arguments for the expr operation.
#[derive(Debug, Args)]
pub struct ExprArgs {
    /// 引数は1つのみ受け取ります。計算したい式をダブルクォートで囲ってください。
    #[arg(required = true, value_hint = ValueHint::Unknown)]
    args: String,
}

pub fn execute(expr_args: ExprArgs) -> Result<()> {
    if cfg!(debug_assertions) {
        println!("{:?}", expr_args);
    }

    let container = CliServiceContainer::build();

    let req = ExprRequest::new(expr_args.args.split_whitespace().collect());
    req.validate()?;

    let cli_response = cli::controllers::expr_controller::handle(req, container);

    match cli_response.result() {
        Ok(value) => {
            println!("{}", value);
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    Ok(())
}
