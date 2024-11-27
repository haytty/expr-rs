mod containers;
mod entrypoints;
mod interfaces;
mod usecases;

use anyhow::Result;

fn main() -> Result<()> {
    entrypoints::cli::root::execute()
}
