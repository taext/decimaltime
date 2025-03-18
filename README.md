# Decimal Time

([online demonstration](https://v1d.dk/dt/))

<br>

A Rust library that implements a custom date/time format called "Decimal Time" which represents time as a year, day of year, and decimal fraction of the day.

## Concept

Decimal Time simplifies time representation by using:
- Year (YYYY): Standard Gregorian calendar year
- Day of Year (DDD): The ordinal day number (1-366)
- Decimal Day (0.FFFFF): A fraction between 0.0 and 1.0 representing the portion of the day elapsed
  - 0.0 = midnight
  - 0.25 = 6:00 AM
  - 0.5 = noon
  - 0.75 = 6:00 PM

The result is a simpler, more intuitive time format that makes calculations easier while remaining compatible with standard time libraries.

## Examples

```rust
// Standard time: March 14, 2025, 12:00:00 (noon)
// Decimal time: 2025.73.5 (73rd day of 2025, halfway through the day)

// Standard time: January 5, 2025, 18:00:00 (6 PM)
// Decimal time: 2025.5.75 (5th day of 2025, 75% through the day)
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
decimal_time = "0.1.0"
```

## API Reference

### DecimalTime Struct

```rust
pub struct DecimalTime {
    pub year: i32,
    pub day_of_year: u32,
    pub decimal_day: f64,
}
```

### Creating Decimal Time

```rust
// Create a new DecimalTime instance
let dt = DecimalTime::new(2025, 73, 0.5);  // March 14, 2025, 12:00 PM

// Convert from chrono's NaiveDateTime
let naive_dt = chrono::NaiveDate::from_ymd_opt(2025, 3, 14).unwrap()
    .and_hms_opt(12, 0, 0).unwrap();
let dt = DecimalTime::from_naive_datetime(naive_dt);

// Convert from chrono's DateTime<Utc>
let utc_dt = chrono::Utc.with_ymd_and_hms(2025, 3, 14, 12, 0, 0).unwrap();
let dt = DecimalTime::from_datetime_utc(utc_dt);
```

### Converting to Standard DateTime

```rust
// Convert to chrono's NaiveDateTime
let naive_dt = dt.to_naive_datetime();

// Convert to chrono's DateTime<Utc>
let utc_dt = dt.to_datetime_utc();
```

### Formatting

```rust
let dt = DecimalTime::new(2025, 73, 0.5);

// Format using placeholders
dt.format("Year: %Y, Day: %d, Time: %f");  // "Year: 2025, Day: 073, Time: 0.5"

// Format using compact decimal notation
dt.format("%Y.%D.%F");  // "2025.73.5"
```

### Format Specifiers

- `%Y` - Full year (e.g., "2025")
- `%d` - Day of year, zero-padded to 3 digits (e.g., "073")
- `%D` - Day of year, not padded (e.g., "73")
- `%f` - Decimal fraction of day with leading "0." (e.g., "0.5")
- `%F` - Decimal fraction of day without the "0." prefix (e.g., "5")

## Examples

### Basic Usage

```rust
use decimal_time::DecimalTime;

// Create a time representing noon on the 73rd day of 2025
let dt = DecimalTime::new(2025, 73, 0.5);

// Format as standard representation
println!("{}", dt.format("%Y-%d %f"));  // "2025-073 0.5"

// Format as compact decimal notation
println!("{}", dt.format("%Y.%D.%F"));  // "2025.73.5"
```

### Converting Between Time Formats

```rust
use decimal_time::DecimalTime;
use chrono::{NaiveDate, Utc};

// From standard time to decimal time
let naive_dt = NaiveDate::from_ymd_opt(2025, 3, 14).unwrap()
    .and_hms_opt(12, 0, 0).unwrap();
let dt = DecimalTime::from_naive_datetime(naive_dt);
println!("{}", dt.format("%Y.%D.%F"));  // "2025.73.5"

// From decimal time back to standard time
let standard_time = dt.to_naive_datetime();
println!("{}", standard_time);  // "2025-03-14T12:00:00"
```

## License

This crate is licensed under the MIT License.