use decimal_time::DecimalTime;
use chrono::{TimeZone, NaiveDate, Utc};
use chrono::{Datelike, Timelike};


/// ✅ Test creating a valid DecimalTime
#[test]
fn test_new_valid() {
    let dec = DecimalTime::new(2025, 100, 0.25);
    assert_eq!(dec.year, 2025);
    assert_eq!(dec.day_of_year, 100);
    assert!((dec.decimal_day - 0.25).abs() < f64::EPSILON);
}

/// ❌ Test invalid `decimal_day` values
#[test]
#[should_panic]
fn test_invalid_decimal_day_too_high() {
    DecimalTime::new(2025, 50, 1.0); // Should panic
}

#[test]
#[should_panic]
fn test_invalid_decimal_day_negative() {
    DecimalTime::new(2025, 50, -0.1); // Should panic
}

/// ❌ Test invalid `day_of_year` values
#[test]
#[should_panic]
fn test_invalid_day_zero() {
    DecimalTime::new(2025, 0, 0.5); // Should panic
}

#[test]
#[should_panic]
fn test_invalid_day_too_high() {
    DecimalTime::new(2025, 367, 0.5); // Should panic (normal year)
}

/// ✅ Test leap year handling (valid and invalid cases)
#[test]
fn test_leap_year_valid_366() {
    let _ = DecimalTime::new(2024, 366, 0.5); // 2024 is a leap year, should be fine
}

// #[test]
// #[should_panic]
// fn test_non_leap_year_day_366() {
//     DecimalTime::new(2025, 366, 0.5); // 2025 is NOT a leap year
// }

/// ✅ Test conversion from `NaiveDateTime`
#[test]
fn test_from_naive_datetime() {
    let dt = NaiveDate::from_ymd_opt(2025, 3, 14).unwrap()
        .and_hms_micro_opt(12, 0, 0, 0).unwrap();

    let dec = DecimalTime::from_naive_datetime(dt);

    assert_eq!(dec.year, 2025);
    assert_eq!(dec.day_of_year, 73); // March 14 is day 73
    assert!((dec.decimal_day - 0.5).abs() < f64::EPSILON); // 12:00:00 is halfway
}

/// ✅ Test conversion from `DateTime<Utc>`
#[test]
fn test_from_datetime_utc() {
    let dt = Utc.with_ymd_and_hms(2025, 3, 14, 6, 0, 0).unwrap();
    let dec = DecimalTime::from_datetime_utc(dt);

    assert_eq!(dec.year, 2025);
    assert_eq!(dec.day_of_year, 73);
    assert!((dec.decimal_day - 0.25).abs() < f64::EPSILON); // 06:00 is 0.25 of the day
}

/// ✅ Test conversion back to `NaiveDateTime`
#[test]
fn test_to_naive_datetime() {
    let dec = DecimalTime::new(2025, 73, 0.5);
    let dt = dec.to_naive_datetime();

    assert_eq!(dt.date(), NaiveDate::from_ymd_opt(2025, 3, 14).unwrap());
    assert_eq!(dt.hour(), 12);
    assert_eq!(dt.minute(), 0);
    assert_eq!(dt.second(), 0);
}

/// ✅ Test conversion back to `DateTime<Utc>`
#[test]
fn test_to_datetime_utc() {
    let dec = DecimalTime::new(2025, 73, 0.75);
    let dt = dec.to_datetime_utc();

    assert_eq!(dt.year(), 2025);
    assert_eq!(dt.ordinal(), 73);
    assert_eq!(dt.hour(), 18);
    assert_eq!(dt.minute(), 0);
    assert_eq!(dt.second(), 0);
}

/// ✅ Test formatting
#[test]
fn test_format() {
    let dec = DecimalTime::new(2025, 100, 0.123456);
    let formatted = dec.format("Year=%Y Day=%d Fraction=%f");

    assert_eq!(formatted, "Year=2025 Day=100 Fraction=0.123456");
}

/// ✅ Test full round-trip conversion (UTC -> Decimal -> UTC)
#[test]
fn test_round_trip_consistency() {
    let dt = Utc.with_ymd_and_hms(2025, 12, 31, 23, 59, 59).unwrap();
    let dec = DecimalTime::from_datetime_utc(dt);
    let back_dt = dec.to_datetime_utc();

    assert_eq!(dt, back_dt, "Round-trip conversion failed");
}

/// ✅ Test 00:00:00 maps to decimal 0.0
#[test]
fn test_midnight() {
    let dt = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let dec = DecimalTime::from_datetime_utc(dt);

    assert_eq!(dec.decimal_day, 0.0);
}

/// ✅ Test 23:59:59 maps close to 1.0 but never equals 1.0
#[test]
fn test_end_of_day() {
    let dt = Utc.with_ymd_and_hms(2025, 1, 1, 23, 59, 59).unwrap();
    let dec = DecimalTime::from_datetime_utc(dt);

    assert!(dec.decimal_day < 1.0, "decimal_day should always be < 1.0");
    assert!((dec.decimal_day - 0.99999).abs() < 0.0001);
}

/// ✅ Test that a difference in time correctly maps to decimal fraction difference
#[test]
fn test_time_fraction_consistency() {
    let dt1 = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let dt2 = Utc.with_ymd_and_hms(2025, 1, 1, 12, 0, 0).unwrap();

    let dec1 = DecimalTime::from_datetime_utc(dt1);
    let dec2 = DecimalTime::from_datetime_utc(dt2);

    assert!((dec2.decimal_day - dec1.decimal_day - 0.5).abs() < f64::EPSILON);
}

/// ✅ Test small increments (1 second accuracy)
#[test]
fn test_seconds_accuracy() {
    let dt1 = Utc.with_ymd_and_hms(2025, 1, 1, 12, 0, 0).unwrap();
    let dt2 = Utc.with_ymd_and_hms(2025, 1, 1, 12, 0, 1).unwrap(); // One second later

    let dec1 = DecimalTime::from_datetime_utc(dt1);
    let dec2 = DecimalTime::from_datetime_utc(dt2);

    let diff = dec2.decimal_day - dec1.decimal_day;
    let expected_diff = 1.0 / 86_400.0; // One second in decimal day

    assert!((diff - expected_diff).abs() < f64::EPSILON, "1 second shift failed");
}
