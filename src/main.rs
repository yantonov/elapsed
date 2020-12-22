use crate::cli::{Command, SinceFormat};
use colored::Colorize;
use crate::elapsed::FormatType;

mod cli;
mod elapsed;

fn entry_point() -> Result<(), String> {
    let arguments = cli::arguments();
    match arguments.command() {
        Command::Since(since) => {
            let from = since.get_from()?;
            let to = since.get_to()?;
            let result = since.format()?;
            println!("{}", elapsed::elapsed(&from, &to)
                .unwrap()
                .format(&match result {
                    SinceFormat::Day => FormatType::Day,
                    SinceFormat::YearDay => FormatType::YearDay,
                    SinceFormat::YearMonth => FormatType::YearMonth,
                    SinceFormat::Default => FormatType::Default
                }));
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
