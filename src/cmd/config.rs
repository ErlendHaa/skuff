use chrono::Local;
use chrono::NaiveDate;
use chrono::NaiveTime;

use skuff::util::parse_date;
use skuff::util::parse_time;
use skuff::Config;
use skuff::ConfigArgs;
use skuff::Session;
use skuff::Store;


/// Start a new session
#[derive(clap::Args)]
pub struct Cmd {
    #[command(flatten)]
    config: ConfigArgs,

    #[arg(short, long, value_parser = parse_time, default_value_t = Local::now().time())]
    time: NaiveTime,

    #[arg(short, long, value_parser = parse_date, default_value_t = Local::now().date_naive())]
    date: NaiveDate,
}

pub fn run(args: Cmd) -> Result<(), String> {
    let date = args.date;
    let time = args.time;
    let config = Config::from_args(args.config)?;

    let dt = date.and_time(time);

    let session = Session::begin(dt);

    Store::from_config(&config)?
        .insert(session)
}
