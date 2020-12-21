use clap::{Clap, crate_version};
use std::str::FromStr;

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

pub enum SinceFormat {
    Days,
    YearDay,
    YearMonth,
    Default,
}

impl FromStr for SinceFormat {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "days" => Ok(SinceFormat::Days),
            "year-day" => Ok(SinceFormat::YearDay),
            "year-month" => Ok(SinceFormat::YearMonth),
            "default" => Ok(SinceFormat::Default),
            _ => Err(format!("invalid format: {}", value))
        }
    }
}

#[derive(Clap)]
pub struct Since {
    #[clap(about = "format YYYY-MM-DD")]
    pub date: String,

    #[clap(about = "days | year-day | year-month | default")]
    pub format: Option<String>,

    pub now: Option<String>,
}

impl Since {
    pub fn format(&self) -> Result<SinceFormat, String> {
        match &self.format {
            None => Ok(SinceFormat::Default),
            Some(x) => {
                SinceFormat::from_str(x)
            }
        }
    }
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