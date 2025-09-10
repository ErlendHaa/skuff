use std::path::PathBuf;

use skuff::Error;
use skuff::util::*;

/// Create a new skuff-stream
#[derive(clap::Args)]
pub struct Args {
    #[arg(long)]
    storage: Option<PathBuf>,
}

pub fn command(args: Args) -> Result<(), Error> {
    let storage = storage(args.storage)?;
    let streams = storage.streams()?;
    let current = storage.current_stream()?;

    for stream in &streams {
        let prefix = if Some(stream) == current.as_ref() {
            "* "
        } else {
            "  "
        };
        println!("{}{}", prefix, stream);
    }

    Ok(())
}
