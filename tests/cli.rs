//! End to end tests: every case runs the real binary and inspects its
//! stdout / stderr / exit code, so a change of the cli interface itself
//! (and not only of the calculation) is detected.

use std::process::{Command, Output};

const EXECUTABLE: &str = env!("CARGO_BIN_EXE_elapsed");

const SUCCESS: i32 = 0;
const APPLICATION_ERROR: i32 = 1;
const USAGE_ERROR: i32 = 2;

fn run(arguments: &[&str]) -> Output {
    Command::new(EXECUTABLE)
        .args(arguments)
        .output()
        .unwrap_or_else(|error| panic!("cannot run {}: {}", EXECUTABLE, error))
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).trim().to_string()
}

/// Runs the binary, checks the expected exit code and returns the output
/// stream which is meaningful for that code.
fn output_for(arguments: &[&str], expected_code: i32) -> String {
    let output = run(arguments);
    assert!(
        !stderr(&output).contains("panicked"),
        "{:?} panicked:\n{}",
        arguments,
        stderr(&output)
    );
    assert_eq!(
        Some(expected_code),
        output.status.code(),
        "unexpected exit code for {:?}\nstdout: {}\nstderr: {}",
        arguments,
        stdout(&output),
        stderr(&output)
    );
    match expected_code {
        SUCCESS => stdout(&output),
        _ => {
            assert_eq!(
                "",
                stdout(&output),
                "nothing should be printed to stdout for {:?}",
                arguments
            );
            stderr(&output)
        }
    }
}

fn elapsed(arguments: &[&str]) -> String {
    output_for(arguments, SUCCESS)
}

fn application_error(arguments: &[&str]) -> String {
    output_for(arguments, APPLICATION_ERROR)
}

fn usage_error(arguments: &[&str]) -> String {
    output_for(arguments, USAGE_ERROR)
}

/// The date is a positional argument, no flag is required to pass it.
/// Regression: the clap 4 migration turned it into a mandatory -d/--date flag,
/// so every documented invocation failed with
/// 'the following required arguments were not provided: --date <DATE>'.
#[test]
fn date_is_a_positional_argument() {
    assert_eq!("1 day", elapsed(&["since", "2020-12-01", "2020-12-03"]));

    let message = usage_error(&["since", "--date", "2020-12-01", "2020-12-03"]);
    assert!(
        message.contains("unexpected argument '--date'"),
        "the date is expected to be positional only, got:\n{}",
        message
    );
}

/// Regression: the date used to be parsed as a DateTime with a fixed offset,
/// which requires a timezone in the input, so every YYYY-MM-DD date was
/// rejected as malformed even when it was passed correctly.
#[test]
fn plain_date_without_timezone_is_accepted() {
    assert_eq!("5 days", elapsed(&["since", "2020-05-04", "2020-05-10"]));
}

#[test]
fn default_format() {
    let cases = [
        ("2020-12-01", "2020-12-01", "0 days"),
        ("2020-12-01", "2020-12-02", "0 days"),
        ("2020-12-01", "2020-12-03", "1 day"),
        ("2020-05-04", "2020-05-10", "5 days"),
        ("2019-12-31", "2021-01-03", "1 year 2 days (368 days)"),
        ("2020-01-03", "2021-01-02", "11 months 29 days (364 days)"),
        ("2020-01-03", "2021-01-31", "11 months 58 days (393 days)"),
        ("2020-01-03", "2021-02-01", "1 year 28 days (394 days)"),
    ];
    for (date, now, expected) in cases {
        assert_eq!(
            expected,
            elapsed(&["since", date, now]),
            "since {} {}",
            date,
            now
        );
        assert_eq!(
            expected,
            elapsed(&["since", date, now, "--format", "default"]),
            "since {} {} --format default",
            date,
            now
        );
    }
}

#[test]
fn explicit_formats() {
    let date = "2015-01-02";
    let now = "2020-11-21";
    let cases = [
        ("day", "2149 days"),
        ("year-day", "5 years 324 days"),
        ("year-month", "5 years 9 months 49 days"),
        ("default", "5 years 9 months 49 days (2149 days)"),
    ];
    for (format, expected) in cases {
        assert_eq!(
            expected,
            elapsed(&["since", date, now, "--format", format]),
            "--format {}",
            format
        );
        assert_eq!(
            expected,
            elapsed(&["since", date, now, "-f", format]),
            "-f {}",
            format
        );
    }
}

/// The format flag is independent of the positional arguments.
#[test]
fn format_flag_can_be_passed_before_the_dates() {
    assert_eq!(
        "2149 days",
        elapsed(&["since", "--format", "day", "2015-01-02", "2020-11-21"])
    );
}

#[test]
fn current_date_is_used_when_now_is_omitted() {
    let default_format = elapsed(&["since", "2015-01-02"]);
    let total_days = elapsed(&["since", "2015-01-02", "--format", "day"]);
    assert_ne!(
        "0 days", total_days,
        "the current date is expected to be later than 2015-01-02"
    );
    // both runs are expected to use the same current date
    assert!(
        default_format.ends_with(&format!("({})", total_days)),
        "inconsistent output: {} / {}",
        default_format,
        total_days
    );
}

#[test]
fn malformed_date_is_reported() {
    let malformed = [
        "2020-13-01",
        "2020-01-32",
        "01-02-2020",
        "2020/01/02",
        "today",
        "",
    ];
    for date in malformed {
        assert_eq!(
            "[ERROR] Date should follow the YYYY-MM-DD format",
            application_error(&["since", date]),
            "as the date: {:?}",
            date
        );
        assert_eq!(
            "[ERROR] Date should follow the YYYY-MM-DD format",
            application_error(&["since", "2020-01-01", date]),
            "as the current date: {:?}",
            date
        );
    }
}

#[test]
fn reversed_dates_are_reported() {
    assert_eq!(
        "[ERROR] 'from' date should be less or equal to 'to' date",
        application_error(&["since", "2021-01-01", "2020-01-01"])
    );
}

#[test]
fn unknown_format_is_reported() {
    assert_eq!(
        "[ERROR] invalid format: month",
        application_error(&["since", "2020-01-01", "2020-02-01", "--format", "month"])
    );
}

#[test]
fn missing_date_is_reported() {
    let message = usage_error(&["since"]);
    assert!(
        message.contains("<DATE>"),
        "the missing date is expected to be mentioned, got:\n{}",
        message
    );
}

#[test]
fn extra_positional_argument_is_reported() {
    usage_error(&["since", "2020-01-01", "2020-01-02", "2020-01-03"]);
}

#[test]
fn help_describes_the_since_command() {
    let help = elapsed(&["since", "--help"]);
    assert!(
        help.contains("Usage:") && help.contains("<DATE>") && help.contains("[NOW]"),
        "unexpected usage line:\n{}",
        help
    );
    assert!(
        help.contains("YYYY-MM-DD"),
        "the expected date format is not documented:\n{}",
        help
    );
    assert!(
        help.contains("-f, --format"),
        "the format flag is not documented:\n{}",
        help
    );
}

/// The binary reports the version together with the full hash of the commit
/// it was built from, so a binary found on a machine can be traced back
/// to the exact sources.
#[test]
fn version_command_shows_the_version_and_the_full_commit_hash() {
    let output = elapsed(&["version"]);

    let (name, version_and_hash) = output
        .split_once(' ')
        .unwrap_or_else(|| panic!("unexpected version output: {}", output));
    assert_eq!(env!("CARGO_PKG_NAME"), name);

    let (version, hash) = version_and_hash
        .split_once(' ')
        .unwrap_or_else(|| panic!("the commit hash is missing: {}", output));
    assert_eq!(env!("CARGO_PKG_VERSION"), version);

    let hash = hash
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
        .unwrap_or_else(|| panic!("the commit hash is not parenthesized: {}", output));
    // the full hash, not the abbreviated one
    assert_eq!(
        40,
        hash.len(),
        "the full commit hash is expected, got: {}",
        hash
    );
    assert!(
        hash.chars().all(|c| c.is_ascii_hexdigit()),
        "not a commit hash: {}",
        hash
    );
}

/// The flag and the command are two spellings of the same thing.
#[test]
fn version_flag_matches_the_version_command() {
    let expected = elapsed(&["version"]);
    for flag in ["--version", "-V"] {
        assert_eq!(expected, elapsed(&[flag]), "{}", flag);
    }
}

#[test]
fn help_mentions_the_version_command() {
    let help = elapsed(&["--help"]);
    assert!(
        help.contains("version"),
        "the version command is not documented:\n{}",
        help
    );
}

#[test]
fn version_command_takes_no_arguments() {
    usage_error(&["version", "2020-01-01"]);
}
