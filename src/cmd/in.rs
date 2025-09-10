use std::path::PathBuf;

use chrono::NaiveDate;
use chrono::NaiveTime;

use chrono::Utc;
use skuff::Entity;
use skuff::Error;
use skuff::Event;
use skuff::Id;
use skuff::util::*;

/// Register an "in" event
#[derive(clap::Args)]
pub struct Args {
    #[arg(short, long, value_parser = parse_time, default_value_t = clock())]
    time: NaiveTime,

    #[arg(short, long, value_parser = parse_date, default_value_t = today())]
    date: NaiveDate,

    #[arg(long)]
    config_file: Option<PathBuf>,

    #[arg(long)]
    stream: Option<String>,

    #[arg(long)]
    storage: Option<PathBuf>,
}

pub fn command(args: Args) -> Result<(), Error> {
    let storage = storage(args.storage)?;

    let event = Event::Create {
        id: Id::new(),
        created_at: Utc::now(),
        entity: Entity::Login {
            id: Id::new(),
            timestamp: from_naive(&args.date, &args.time),
        },
    };

    storage.stream_append(event, &args.stream)?;

    Ok(())
}
