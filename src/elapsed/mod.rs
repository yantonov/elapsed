use chrono;
use chrono::{NaiveDate, Datelike};
use std::cmp::max;

pub type YearImpl = i32;
pub type MonthImpl = u32;
pub type DayImpl = u32;

pub struct Duration {
    year: YearImpl,
    month: MonthImpl,
    day: DayImpl,
}

impl Duration {
    pub fn year(&self) -> YearImpl {
        return self.year;
    }

    pub fn month(&self) -> MonthImpl {
        return self.month;
    }

    pub fn day(&self) -> DayImpl {
        return self.day;
    }
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

impl ToString for Duration {
    fn to_string(&self) -> String {
        if self.year == 0 && self.month == 0 && self.day == 0 {
            "0 days".to_string()
        } else {
            let tokens: Vec<String> = vec![
                format_num(self.year as i32, "year", "years"),
                format_num(self.month as i32, "month", "months"),
                format_num(self.day as i32, "day", "days")
            ].into_iter()
                .filter(|x| !x.is_empty())
                .collect();
            tokens.join(" ")
        }
    }
}

fn month_difference(from: &NaiveDate, to: &NaiveDate) -> MonthImpl {
    if from.year() == to.year() {
        return max(0, to.month() as i32 - 1 - (from.month() as i32 + 1) + 1) as MonthImpl;
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
                NaiveDate::from_ymd(
                    from.year(),
                    from.month() + 1,
                    1)
                    .pred()
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
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year() {
        assert_eq!(2, duration_year(
            2017, 3, 31,
            2020, 2, 20));
        assert_eq!(1, duration_year(
            2018, 3, 31,
            2020, 2, 20));
        assert_eq!(0, duration_year(
            2019, 3, 31,
            2020, 2, 20));
        assert_eq!(0, duration_year(
            2020, 1, 1,
            2020, 2, 20));
        assert_eq!(2, duration_year(
            2018, 2, 3,
            2020, 8, 2));
        assert_eq!(1, duration_year(
            2020, 1, 3,
            2021, 2, 1));
    }

    #[test]
    fn month() {
        let year = 2020;
        assert_eq!(0, duration_month(
            year, 2, 3,
            year, 2, 4));
        assert_eq!(0, duration_month(
            year, 2, 3,
            year, 3, 2));
        assert_eq!(1, duration_month(
            year, 2, 3,
            year, 4, 2));
        assert_eq!(5, duration_month(
            year, 2, 3,
            year, 8, 2));
        assert_eq!(10 + 7 - 12, duration_month(
            2018, 2, 3,
            2020, 8, 2));
        assert_eq!(0, duration_month(
            2020, 1, 3,
            2021, 2, 1));
    }

    #[test]
    fn day() {
        assert_eq!(0, duration_day(
            2020, 2, 1,
            2020, 2, 1));
        assert_eq!(0, duration_day(
            2020, 2, 1,
            2020, 2, 2));
        assert_eq!(6, duration_day(
            2020, 2, 3,
            2020, 2, 10));
        assert_eq!(29 - 4 + 1 + 9, duration_day(
            2020, 2, 3,
            2020, 3, 10));
        assert_eq!(31 - 4 + 1 + 9, duration_day(
            2020, 3, 3,
            2020, 4, 10));
        assert_eq!(31 - 4 + 1 + 9, duration_day(
            2020, 12, 3,
            2021, 1, 10));
    }

    #[test]
    fn to_string() {
        assert_eq!("0 days",
                   Duration {
                       year: 0,
                       month: 0,
                       day: 0,
                   }.to_string());
        assert_eq!("1 day",
                   Duration {
                       year: 0,
                       month: 0,
                       day: 1,
                   }.to_string());
        assert_eq!("2 days",
                   Duration {
                       year: 0,
                       month: 0,
                       day: 2,
                   }.to_string());
        assert_eq!("1 month 1 day",
                   Duration {
                       year: 0,
                       month: 1,
                       day: 1,
                   }.to_string());
        assert_eq!("1 month 12 days",
                   Duration {
                       year: 0,
                       month: 1,
                       day: 12,
                   }.to_string());
        assert_eq!("2 months 12 days",
                   Duration {
                       year: 0,
                       month: 2,
                       day: 12,
                   }.to_string());
        assert_eq!("1 year 12 days",
                   Duration {
                       year: 1,
                       month: 0,
                       day: 12,
                   }.to_string());
        assert_eq!("2 years 2 months 12 days",
                   Duration {
                       year: 2,
                       month: 2,
                       day: 12,
                   }.to_string());
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

    fn duration_year(from_year: YearImpl, from_month: MonthImpl, from_day: DayImpl,
                     to_year: YearImpl, to_month: MonthImpl, to_day: DayImpl) -> YearImpl {
        duration(from_year, from_month, from_day,
                 to_year, to_month, to_day)
            .unwrap()
            .year()
    }

    fn duration_month(from_year: YearImpl, from_month: MonthImpl, from_day: DayImpl,
                      to_year: YearImpl, to_month: MonthImpl, to_day: DayImpl) -> MonthImpl {
        duration(from_year, from_month, from_day,
                 to_year, to_month, to_day)
            .unwrap()
            .month()
    }

    fn duration_day(from_year: YearImpl, from_month: MonthImpl, from_day: DayImpl,
                    to_year: YearImpl, to_month: MonthImpl, to_day: DayImpl) -> MonthImpl {
        duration(from_year, from_month, from_day,
                 to_year, to_month, to_day)
            .unwrap()
            .day()
    }
}