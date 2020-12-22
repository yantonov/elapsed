use clap::{Clap, crate_version};
use std::str::FromStr;
use chrono::{NaiveDate, Utc, TimeZone};

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

    #[clap(about = "days | year-day | year-month | default", short, long,)]
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

    pub fn get_from(&self) -> Result<NaiveDate, String> {
        let parsed_from_date = Utc.datetime_from_str(
            &format!("{} 00:00:00", self.date),
            "%Y-%m-%d %H:%M:%S")
            .map_err(|_| "Date should follow the YYYY-MM-DD format".to_string())?;
        Ok(parsed_from_date
            .date()
            .naive_local())
    }

    pub fn get_to(&self) -> Result<NaiveDate, String> {
        Ok(match &self.now {
            None => Utc::now().date().naive_local(),
            Some(now_value) => {
                let parsed_to_date = Utc.datetime_from_str(
                    &format!("{} 00:00:00", now_value),
                    "%Y-%m-%d %H:%M:%S")
                    .map_err(|_| "Date should follow the YYYY-MM-DD format".to_string())?;
                parsed_to_date
                    .date()
                    .naive_local()
            }
        })
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