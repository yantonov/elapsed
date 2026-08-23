use crate::version::VERSION_WITH_COMMIT_HASH;
use chrono::{NaiveDate, Utc};
use clap::Parser;
use std::str::FromStr;

#[derive(Parser)]
#[clap(version = VERSION_WITH_COMMIT_HASH)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
pub enum Command {
    #[clap(about = "calculate elapsed time since given date", display_order = 0)]
    Since(Since),

    #[clap(
        about = "show the version and the commit hash of this binary",
        display_order = 1
    )]
    Version,
}

pub enum SinceFormat {
    Day,
    YearDay,
    YearMonth,
    Default,
}

impl FromStr for SinceFormat {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "day" => Ok(SinceFormat::Day),
            "year-day" => Ok(SinceFormat::YearDay),
            "year-month" => Ok(SinceFormat::YearMonth),
            "default" => Ok(SinceFormat::Default),
            _ => Err(format!("invalid format: {}", value)),
        }
    }
}

#[derive(Parser)]
#[clap(about)]
pub struct Since {
    /// format YYYY-MM-DD
    pub date: String,

    /// format YYYY-MM-DD, current date is used by default
    pub now: Option<String>,

    /// day | year-day | year-month | default
    #[arg(short, long)]
    pub format: Option<String>,
}

impl Since {
    pub fn format(&self) -> Result<SinceFormat, String> {
        match &self.format {
            None => Ok(SinceFormat::Default),
            Some(x) => SinceFormat::from_str(x),
        }
    }

    fn parse_date(&self, date: &str) -> Result<NaiveDate, String> {
        NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|_| "Date should follow the YYYY-MM-DD format".to_string())
    }

    pub fn get_from(&self) -> Result<NaiveDate, String> {
        self.parse_date(&self.date)
    }

    pub fn get_to(&self) -> Result<NaiveDate, String> {
        match &self.now {
            None => Ok(Utc::now().date_naive()),
            Some(now_value) => self.parse_date(now_value),
        }
    }
}

pub struct Arguments {
    args: Opts,
}

impl Arguments {
    pub fn command(&self) -> &Command {
        &self.args.command
    }
}

pub fn arguments() -> Arguments {
    Arguments {
        args: Opts::parse(),
    }
}
