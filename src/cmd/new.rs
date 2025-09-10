use std::path::PathBuf;

use skuff::Error;
use skuff::util::*;

/// Create a new skuff-stream
#[derive(clap::Args)]
pub struct Args {
    #[arg(short, long, default_value_t = true)]
    set_current: bool,

    #[arg(value_parser = validate_stream)]
    stream: String,

    #[arg(long)]
    storage: Option<PathBuf>,
}

pub fn command(args: Args) -> Result<(), Error> {
    let storage = storage(args.storage)?;

    let _ = storage.stream_create(&args.stream)?;
    println!("Created stream: {}", args.stream);

    if args.set_current {
        storage.set_current_stream(&args.stream)?;
        println!("Current stream is: {}", args.stream);
    }

    Ok(())
}
