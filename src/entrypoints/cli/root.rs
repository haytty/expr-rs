use crate::entrypoints::cli::completion::CompletionArgs;
use crate::entrypoints::cli::expr::ExprArgs;
use crate::entrypoints::cli::{completion, expr};
use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};

/// Main CLI structure for the expr-fs application.
#[derive(Parser, Debug)]
#[command(version, about, long_about = "Easy Expring for Rust Projects")]
struct Cli {
    /// expr-fs subcommands
    #[command(subcommand)]
    command: SubCommands,
}

/// Enum representing the available subcommands for the CLI.
#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// Expr project directories and files.
    Expr(ExprArgs),

    /// Generate completion scripts for the specified shell.
    Completion(CompletionArgs),
}

pub fn execute() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        SubCommands::Expr(expr_args) => expr::execute(expr_args),
        SubCommands::Completion(completion_args) => {
            completion::execute(completion_args, Cli::command())
        }
    }
}
