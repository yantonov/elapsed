use crate::cli::Command;
use chrono::{Utc, TimeZone, NaiveDate};
use colored::Colorize;

mod cli;
mod elapsed;

fn get_from(date: &String) -> Result<NaiveDate, String> {
    let parsed_from_date = Utc.datetime_from_str(
        &format!("{} 00:00:00", date),
        "%Y-%m-%d %H:%M:%S")
        .map_err(|_| "Date should follow the YYYY-MM-DD format".to_string())?;
    Ok(parsed_from_date
        .date()
        .naive_local())
}

fn get_to(now: &Option<String>) -> Result<NaiveDate, String> {
    Ok(match now {
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

fn entry_point() -> Result<(), String> {
    let arguments = cli::arguments();
    match arguments.command() {
        Command::Since(since) => {
            let from = get_from(&since.date)?;
            let to = get_to(&since.now)?;
            println!("{}", elapsed::elapsed(&from, &to).unwrap().to_string());
        }
    }
    Ok(())
}

fn main() {
    match entry_point() {
        Ok(_) => {
            std::process::exit(0);
        }
        Err(message) => {
            eprintln!("{} {}", "[ERROR]".red(), message);
            std::process::exit(1);
        }
    }
}
