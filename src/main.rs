use clap::Parser;
use colored;

mod cmd;

const ABOUT: &str = r#"Skuff "#;

fn main() -> Result<(), skuff::Error> {
    let cli = Cli::parse();

    // Enable colored output
    //
    // This is needed to force colored output on when used in combination with the pager crate
    // which spawns a child process and pipes the output to it.
    //
    // https://github.com/colored-rs/colored/issues/69
    colored::control::set_override(true);

    use Command::*;
    match cli.command {
        New(args) => cmd::new::command(args),
        In(args) => cmd::r#in::command(args),
        Out(args) => cmd::out::command(args),
        Log(args) => cmd::log::command(args),
        Ls(args) => cmd::ls::command(args),
        Rm(args) => cmd::rm::command(args),
        Switch(args) => cmd::switch::command(args),
    }
}

#[derive(Parser)]
#[command(author, version, about = ABOUT, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    New(cmd::new::Args),
    In(cmd::r#in::Args),
    Out(cmd::out::Args),
    Log(cmd::log::Args),
    Ls(cmd::ls::Args),
    Rm(cmd::rm::Args),
    Switch(cmd::switch::Args),
}
