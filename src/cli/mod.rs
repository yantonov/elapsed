use std::str::FromStr;
use chrono::{NaiveDate, Utc, DateTime};
use clap::Parser;

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
pub enum Command {
    #[clap(about = "calculate elapsed time since given date", display_order = 0)]
    Since(Since)
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
            _ => Err(format!("invalid format: {}", value))
        }
    }
}

#[derive(Parser)]
#[clap(about)]
pub struct Since {
    #[arg(short, long)]
    // format YYYY-MM-DD
    pub date: String,

    #[arg(short, long)]
    // day | year-day | year-month | default"
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

    fn parse_date(&self, date: &str) -> Result<NaiveDate, String> {
        let parsed_from_date = DateTime::parse_from_str(
            &format!("{} 00:00:00", date),
            "%Y-%m-%d %H:%M:%S")
            .map_err(|_| "Date should follow the YYYY-MM-DD format".to_string())?;
        Ok(parsed_from_date
            .date_naive())
    }

    pub fn get_from(&self) -> Result<NaiveDate, String> {
        self.parse_date(&self.date)
    }

    pub fn get_to(&self) -> Result<NaiveDate, String> {
        Ok(match &self.now {
            None => Utc::now().date_naive(),
            Some(now_value) => {
                self.parse_date(now_value).unwrap()
            }
        })
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
    Arguments { args: Opts::parse() }
}