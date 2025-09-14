use std::path::PathBuf;

use chrono::Utc;
use skuff::Error;
use skuff::Event;
use skuff::Id;
use skuff::util::*;

/// Remove an event
#[derive(clap::Args)]
pub struct Args {
    #[arg(value_parser = Id::from_str)]
    id: Id,

    #[arg(long)]
    config_file: Option<PathBuf>,

    #[arg(long)]
    stream: Option<String>,

    #[arg(long)]
    storage: Option<PathBuf>,
}

pub fn command(args: Args) -> Result<(), Error> {
    let storage = storage(args.storage)?;

    let event = Event::Delete {
        id: Id::new(),
        created_at: Utc::now(),
        entity_id: args.id,
    };

    storage.stream_append(event, &args.stream)?;

    Ok(())
}
