use std::path::PathBuf;

use skuff::Error;
use skuff::util::*;

/// Switch skuff-stream
#[derive(clap::Args)]
pub struct Args {
    #[arg(value_parser = validate_stream)]
    stream: String,

    #[arg(long)]
    storage: Option<PathBuf>,
}

pub fn command(args: Args) -> Result<(), Error> {
    let storage = storage(args.storage)?;
    storage.set_current_stream(&args.stream)?;

    println!("Switched to stream: {}", args.stream);

    Ok(())
}
