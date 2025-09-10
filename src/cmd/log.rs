use std::path::PathBuf;

use pager::Pager;
use skuff::Error;
use skuff::log;
use skuff::replay;
use skuff::util::*;

/// Create a new skuff-stream
#[derive(clap::Args)]
pub struct Args {
    #[arg(long)]
    storage: Option<PathBuf>,

    #[arg(value_parser = validate_stream)]
    stream: Option<String>,
}

pub fn command(args: Args) -> Result<(), Error> {
    let storage = storage(args.storage)?;

    let stream = storage.stream(&args.stream)?;
    let state = replay(&stream);

    Pager::with_pager("more -R").setup();
    log::print(&state);

    Ok(())
}
