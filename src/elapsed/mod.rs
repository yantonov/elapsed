use chrono::{NaiveDate, Datelike};
use std::cmp::max;

pub type YearImpl = i32;
pub type MonthImpl = u32;
pub type DayImpl = u32;

pub struct Duration {
    year: YearImpl,
    month: MonthImpl,
    day: DayImpl,
    total_days: DayImpl,
}

fn format_num(value: i32, suffix_single: &str, suffix_plural: &str) -> String {
    match value {
        0 => {
            "".to_string()
        }
        1 => format!("1 {}", suffix_single),
        v => format!("{} {}", v, suffix_plural),
    }
}

trait DurationFormatter {
    fn format(&self, duration: &Duration) -> String;
}

pub enum FormatType {
    Day,
    YearMonth,
    YearDay,
    Default,
}

struct DayFormatter {}

impl DurationFormatter for DayFormatter {
    fn format(&self, duration: &Duration) -> String {
        if duration.total_days == 0 {
            "0 days".to_string()
        } else {
            format_num(duration.total_days as i32, "day", "days")
        }
    }
}

struct YearMonthFormatter {}

impl DurationFormatter for YearMonthFormatter {
    fn format(&self, duration: &Duration) -> String {
        if duration.year == 0 && duration.month == 0 && duration.day == 0 {
            "0 days".to_string()
        } else {
            let tokens: Vec<String> = vec![
                format_num(duration.year as i32, "year", "years"),
                format_num(duration.month as i32, "month", "months"),
                format_num(duration.day as i32, "day", "days"),
            ].into_iter()
                .filter(|x| !x.is_empty())
                .collect();
            tokens.join(" ")
        }
    }
}

struct YearDaysFormatter {}

impl DurationFormatter for YearDaysFormatter {
    fn format(&self, duration: &Duration) -> String {
        match duration.total_days {
            0 => "0 days".to_string(),
            _ => {
                let year_day_count = 365;
                let year = duration.total_days / year_day_count;
                let days = duration.total_days % year_day_count;
                if year == 0 {
                    format_num(days as i32, "day", "days")
                } else if days == 0 {
                    format_num(year as i32, "year", "years")
                } else {
                    format!("{} {}",
                            format_num(year as i32, "year", "years"),
                            format_num(days as i32, "day", "days"))
                }
            }
        }
    }
}

struct DefaultFormatter {}

impl DurationFormatter for DefaultFormatter {
    fn format(&self, duration: &Duration) -> String {
        let tokens: Vec<String> = vec![
            YearMonthFormatter {}.format(duration),
            if (duration.year == 0 && duration.month == 0) || duration.total_days == 0 {
                "".to_string()
            } else {
                format!("({})", DayFormatter {}.format(duration))
            }
        ].into_iter()
            .filter(|x| !x.is_empty())
            .collect();
        tokens.join(" ")
    }
}

impl Duration {
    pub fn format(&self, format_type: &FormatType) -> String {
        let formatter: &dyn DurationFormatter = match format_type {
            FormatType::Day => &DayFormatter {},
            FormatType::YearMonth => &YearMonthFormatter {},
            FormatType::Default => &DefaultFormatter {},
            FormatType::YearDay => &YearDaysFormatter {}
        };
        formatter.format(self)
    }
}

impl ToString for Duration {
    fn to_string(&self) -> String {
        self.format(&FormatType::Default)
    }
}

fn month_difference(from: &NaiveDate, to: &NaiveDate) -> MonthImpl {
    if from.year() == to.year() {
        max(0, to.month() as i32 - 1 - (from.month() as i32 + 1) + 1) as MonthImpl
    } else {
        (max(0, 12 - (from.month() as i32 + 1) + 1)
            + max(0, to.month() as i32 - 1)) as MonthImpl
    }
}

fn day_difference(from: &NaiveDate, to: &NaiveDate) -> DayImpl {
    if from.year() == to.year() && from.month() == to.month() {
        max(0, to.day() as i32 - 1 - (from.day() as i32 + 1) + 1) as DayImpl
    } else {
        (
            max(0, if from.month() == 12 {
                31 - (from.day() as i32 + 1) + 1
            } else {
                NaiveDate::from_ymd_opt(
                    from.year(),
                    from.month() + 1,
                    1)
                    .unwrap()
                    .pred_opt()
                    .unwrap()
                    .day() as i32 - (from.day() as i32 + 1) + 1
            })
                + max(0, to.day() as i32 - 1)) as DayImpl
    }
}

pub fn elapsed(from: &NaiveDate, to: &NaiveDate) -> Result<Duration, String> {
    if from > to {
        return Err("'from' date should be less or equal to 'to' date".to_string());
    }
    let mut month = month_difference(from, to);
    let mut year = max(0, to.year() - 1 - (from.year() + 1) + 1);
    if month >= 12 {
        year += 1;
        month -= 12;
    }

    Ok(Duration {
        year,
        month,
        day: day_difference(from, to),
        total_days: max(0, to
            .pred_opt()
            .unwrap()
            .signed_duration_since(from.clone())
            .num_days()) as DayImpl,
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year_month_format_test() {
        let year = 2020;
        assert_eq!("0 days", year_month_format(
            year, 2, 3,
            year, 2, 4));
        assert_eq!("27 days", year_month_format(
            year, 2, 3,
            year, 3, 2));
        assert_eq!("1 month 27 days", year_month_format(
            year, 2, 3,
            year, 4, 2));
        assert_eq!("5 months 27 days", year_month_format(
            year, 2, 3,
            year, 8, 2));
        assert_eq!("1 year 28 days", year_month_format(
            2020, 1, 3,
            2021, 2, 1));
        assert_eq!("2 years 5 months 26 days", year_month_format(
            2018, 2, 3,
            2020, 8, 2));
    }

    #[test]
    fn days_format_test() {
        assert_eq!("0 days", days_format(
            2020, 2, 1,
            2020, 2, 1));
        assert_eq!("0 days", days_format(
            2020, 2, 1,
            2020, 2, 2));
        assert_eq!("6 days", days_format(
            2020, 2, 3,
            2020, 2, 10));
        assert_eq!("35 days", days_format(
            2020, 2, 3,
            2020, 3, 10));
        assert_eq!("37 days", days_format(
            2020, 3, 3,
            2020, 4, 10));
        assert_eq!("37 days", days_format(
            2020, 12, 3,
            2021, 1, 10));
        assert_eq!("393 days", days_format(
            2020, 1, 3,
            2021, 1, 31));
    }

    #[test]
    fn to_string() {
        assert_eq!("0 days",
                   duration(2020, 1, 1,
                            2020, 1, 1).unwrap().to_string());
        assert_eq!("0 days",
                   duration(2020, 1, 1,
                            2020, 1, 2).unwrap().to_string());
        assert_eq!("1 day",
                   duration(2020, 1, 1,
                            2020, 1, 3).unwrap().to_string());
        assert_eq!("2 days",
                   duration(2020, 1, 1,
                            2020, 1, 4).unwrap().to_string());
        assert_eq!("1 month 1 day (30 days)",
                   duration(2020, 1, 30,
                            2020, 3, 1).unwrap().to_string());
        assert_eq!("1 month 2 days (31 days)",
                   duration(2020, 1, 30,
                            2020, 3, 2).unwrap().to_string());
        assert_eq!("2 months 12 days (72 days)",
                   duration(2020, 1, 30,
                            2020, 4, 12).unwrap().to_string());
        assert_eq!("1 year 12 days (377 days)",
                   duration(2020, 12, 30,
                            2022, 1, 12).unwrap().to_string());
        assert_eq!("2 years 2 months 12 days (801 days)",
                   duration(2020, 12, 30,
                            2023, 3, 12).unwrap().to_string());
        assert_eq!("11 months 58 days (393 days)",
                   duration(2020, 1, 3,
                            2021, 01, 31).unwrap().to_string());
    }

    #[test]
    fn year_days_format_test() {
        assert_eq!("0 days",
                   year_day_format(2020, 1, 1,
                                   2020, 1, 1));
        assert_eq!("0 days",
                   year_day_format(2020, 1, 1,
                                   2020, 1, 2));
        assert_eq!("1 day",
                   year_day_format(2020, 1, 1,
                                   2020, 1, 3));
        assert_eq!("2 days",
                   year_day_format(2020, 1, 1,
                                   2020, 1, 4));
        assert_eq!("30 days",
                   year_day_format(2020, 1, 30,
                                   2020, 3, 1));
        assert_eq!("31 days",
                   year_day_format(2020, 1, 30,
                                   2020, 3, 2));
        assert_eq!("72 days",
                   year_day_format(2020, 1, 30,
                                   2020, 4, 12));
        assert_eq!("1 year 12 days",
                   year_day_format(2020, 12, 30,
                                   2022, 1, 12));
        assert_eq!("2 years 71 days",
                   year_day_format(2020, 12, 30,
                                   2023, 3, 12));
        assert_eq!("1 year 28 days",
                   year_day_format(2020, 1, 3,
                                   2021, 01, 31));
    }

    #[test]
    fn incorrect_date_order() {
        let duration = duration(
            2021, 1, 1,
            2020, 1, 1);
        match duration {
            Ok(_) => {
                panic!("error expected");
            }
            Err(error_message) => {
                assert_eq!("'from' date should be less or equal to 'to' date",
                           error_message)
            }
        }
    }

    fn duration(from_year: YearImpl, from_month: MonthImpl, from_day: DayImpl,
                to_year: YearImpl, to_month: MonthImpl, to_day: DayImpl) -> Result<Duration, String> {
        elapsed(
            &NaiveDate::from_ymd(from_year, from_month, from_day),
            &NaiveDate::from_ymd(to_year, to_month, to_day))
    }

    fn year_month_format(from_year: YearImpl, from_month: MonthImpl, from_day: DayImpl,
                         to_year: YearImpl, to_month: MonthImpl, to_day: DayImpl) -> String {
        duration(from_year, from_month, from_day,
                 to_year, to_month, to_day)
            .unwrap()
            .format(&FormatType::YearMonth)
    }

    fn days_format(from_year: YearImpl, from_month: MonthImpl, from_day: DayImpl,
                   to_year: YearImpl, to_month: MonthImpl, to_day: DayImpl) -> String {
        duration(from_year, from_month, from_day,
                 to_year, to_month, to_day)
            .unwrap()
            .format(&FormatType::Day)
    }

    fn year_day_format(from_year: YearImpl, from_month: MonthImpl, from_day: DayImpl,
                       to_year: YearImpl, to_month: MonthImpl, to_day: DayImpl) -> String {
        elapsed(
            &NaiveDate::from_ymd(from_year, from_month, from_day),
            &NaiveDate::from_ymd(to_year, to_month, to_day))
            .unwrap()
            .format(&FormatType::YearDay)
    }
}