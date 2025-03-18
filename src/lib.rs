//! # Decimal Time
//!
//! A Rust library that implements a custom date/time format called "Decimal Time."

use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, DateTime, Utc};

/// A struct representing a date/time in “Decimal Time”:
///
/// - `year`: full year (e.g., 2025)
/// - `day_of_year`: the day of year (1-based, in [1..=365 or 366])
/// - `decimal_day`: fraction of the day (0.0 <= decimal_day < 1.0)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecimalTime {
    pub year: i32,
    pub day_of_year: u32,
    /// Fraction of the day in [0.0, 1.0). 0.0 = midnight, 0.5 = noon, etc.
    pub decimal_day: f64,
}

impl DecimalTime {
    /// Creates a new `DecimalTime` instance.
    ///
    /// # Panics
    ///
    /// Panics if `decimal_day` is out of [0,1)
    /// or if `day_of_year` is out of 1..=366.
    pub fn new(year: i32, day_of_year: u32, decimal_day: f64) -> Self {
        if !(0.0..1.0).contains(&decimal_day) {
            panic!(
                "`decimal_day` must be in [0,1). Received: {}",
                decimal_day
            );
        }
        if !(1..=366).contains(&day_of_year) {
            panic!(
                "`day_of_year` must be in [1..=366]. Received: {}",
                day_of_year
            );
        }

        DecimalTime {
            year,
            day_of_year,
            decimal_day,
        }
    }

    /// Converts a `chrono::NaiveDateTime` to a `DecimalTime`.
    pub fn from_naive_datetime(dt: NaiveDateTime) -> Self {
        let year = dt.year();
        let day_of_year = dt.ordinal();

        // total seconds in the day
        let sec_in_day = dt.num_seconds_from_midnight();
        let nano = dt.nanosecond();

        // Convert to microseconds
        let total_microseconds = (sec_in_day as u64) * 1_000_000 + (nano / 1_000) as u64;
        // 86,400 seconds in a day => 86,400_000_000 microseconds
        let fraction_of_day = total_microseconds as f64 / 86_400_000_000.0;

        Self::new(year, day_of_year, fraction_of_day)
    }

    /// Converts a UTC `chrono::DateTime<Utc>` into a `DecimalTime`.
    pub fn from_datetime_utc(dt: DateTime<Utc>) -> Self {
        Self::from_naive_datetime(dt.naive_utc())
    }

    /// Converts `DecimalTime` into a `chrono::NaiveDateTime`.
    ///
    /// # Panics
    ///
    /// Panics if the date is invalid (e.g., day_of_year = 366 in a non-leap year).
    pub fn to_naive_datetime(&self) -> NaiveDateTime {
        // Convert year + ordinal day to NaiveDate
        let base_date = NaiveDate::from_yo_opt(self.year, self.day_of_year)
            .unwrap_or_else(|| {
                panic!(
                    "Invalid day_of_year={} for year={}",
                    self.day_of_year, self.year
                )
            });

        let total_microseconds = (self.decimal_day * 86_400_000_000.0).round() as u64;
        let seconds = total_microseconds / 1_000_000;
        let micros = total_microseconds % 1_000_000;

        base_date
            .and_hms_micro_opt(0, 0, 0, 0)
            .unwrap() // safe as it’s midnight
            .checked_add_signed(chrono::Duration::seconds(seconds as i64))
            .unwrap()
            .checked_add_signed(chrono::Duration::microseconds(micros as i64))
            .unwrap()
    }

    /// Converts `DecimalTime` into a UTC `chrono::DateTime<Utc>`.
    pub fn to_datetime_utc(&self) -> DateTime<Utc> {
        let ndt = self.to_naive_datetime();
        // Use the new recommended method:
        DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc)
    }

    /// Format `DecimalTime` with simple placeholders:
    /// - `%Y` => year
    /// - `%d` => day_of_year (3-digit zero-padded)
    /// - `%f` => fraction of day
    ///
    /// # Example
    /// 
    /// ```
    /// let dec = decimal_time::DecimalTime::new(2025, 100, 0.5);
    /// let s = dec.format("Year=%Y Day=%d Fraction=%f");
    /// // => "Year=2025 Day=100 Fraction=0.5"
    /// ```
    pub fn format(&self, fmt_str: &str) -> String {
        let mut output = fmt_str.to_string();

        // year
        output = output.replace("%Y", &self.year.to_string());

        // day_of_year
        let day_str = format!("{}", self.day_of_year);
        output = output.replace("%d", &day_str);

        // decimal fraction
        if output.contains("%f") {
            let frac = format!("{}", self.decimal_day);
            output = output.replace("%f", &frac.trim_start_matches('0'));
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_new_valid() {
        let dec = DecimalTime::new(2025, 100, 0.25);
        assert_eq!(dec.year, 2025);
        assert_eq!(dec.day_of_year, 100);
        assert!((dec.decimal_day - 0.25).abs() < f64::EPSILON);
    }

    #[test]
    #[should_panic]
    fn test_new_day_of_year_0() {
        // day_of_year = 0 should panic
        let _ = DecimalTime::new(2025, 0, 0.2);
    }

    #[test]
    #[should_panic]
    fn test_new_decimal_day_out_of_range() {
        // decimal_day >= 1.0 is invalid
        let _ = DecimalTime::new(2025, 10, 1.0);
    }

    #[test]
    fn test_from_to_naive_datetime() {
        let dt = NaiveDate::from_ymd_opt(2025, 3, 14).unwrap()
            .and_hms_micro_opt(15, 9, 26, 535_897).unwrap();

        let dec = DecimalTime::from_naive_datetime(dt);
        let back = dec.to_naive_datetime();
        assert_eq!(dt, back);
    }

    #[test]
    fn test_from_to_utc() {
        let utc_dt = Utc.with_ymd_and_hms(2025, 3, 14, 15, 9, 26).unwrap()
                        .with_nanosecond(535_897_000).unwrap();
        let dec = DecimalTime::from_datetime_utc(utc_dt);
        let back_utc = dec.to_datetime_utc();
        assert_eq!(utc_dt, back_utc);
    }

    #[test]
    fn test_format() {
        let dec = DecimalTime::new(2025, 5, 0.5);
        let formatted = dec.format("Date => %Y-%d frac:%f");
        assert_eq!("Date => 2025-5 frac:.5", formatted);
    }
}
