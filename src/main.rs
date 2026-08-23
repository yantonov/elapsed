use crate::cli::{Command, SinceFormat};
use crate::elapsed::FormatType;
use colored::Colorize;

mod cli;
mod elapsed;
mod version;

fn entry_point() -> Result<(), String> {
    let arguments = cli::arguments();
    match arguments.command() {
        Command::Since(since) => {
            let from = since.get_from()?;
            let to = since.get_to()?;
            let result = since.format()?;
            println!(
                "{}",
                elapsed::elapsed(&from, &to)?.format(&match result {
                    SinceFormat::Day => FormatType::Day,
                    SinceFormat::YearDay => FormatType::YearDay,
                    SinceFormat::YearMonth => FormatType::YearMonth,
                    SinceFormat::Default => FormatType::Default,
                })
            );
        }
        Command::Version => {
            println!("{}", version::version_info());
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
