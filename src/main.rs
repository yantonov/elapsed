use crate::cli::Command;
use chrono::{Utc, TimeZone};

mod cli;
mod elapsed;

fn main() {
    let arguments = cli::arguments();
    match arguments.command() {
        Command::Since(since) => {
            let from = Utc.datetime_from_str(
                &format!("{} 00:00:00", since.date),
                "%Y-%m-%d %H:%M:%S")
                .expect("Date should follow the YYYY-MM-DD format")
                .date()
                .naive_local();
            let to = Utc::now().date().naive_local();
            println!("{}", elapsed::elapsed(&from, &to).unwrap().to_string());
        }
    }
}
