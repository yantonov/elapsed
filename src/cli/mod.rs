use clap::{Clap, crate_version};

#[derive(Clap)]
#[clap(version = crate_version ! ())]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap)]
pub enum Command {
    #[clap(about = "calculate elapsed time since given date", display_order = 0)]
    Since(Since)
}

#[derive(Clap)]
pub struct Since {
    #[clap(about = "format YYYY-MM-DD")]
    pub date: String,

    pub now: Option<String>,
}

pub struct Arguments {
    args: Opts
}

impl Arguments {
    pub fn command(&self) -> &Command {
        &self.args.command
    }
}

pub fn arguments() -> Arguments {
    return Arguments { args: Opts::parse() };
}