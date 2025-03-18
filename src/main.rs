// src/main.rs

use chrono::{DateTime, Utc, FixedOffset};
use decimal_time::DecimalTime;

fn main() {
    // Get current UTC time
    let utc_now: DateTime<Utc> = Utc::now();
    
    // Convert to CET (UTC+1)
    let cet1 = FixedOffset::east_opt(1 * 3600).expect("Valid timezone offset");
    let cet1_now = utc_now.with_timezone(&cet1);
    
    // Convert to DecimalTime
    let dec_time = DecimalTime::from_naive_datetime(cet1_now.naive_local());
    
    // Format using DecimalTime's custom formatter
    println!("Right now in Decimal Time (DT): {}", dec_time.format("%Y.%d%f"));
}