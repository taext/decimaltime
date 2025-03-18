// src/main.rs
fn main() {
    println!("DecimalTime crate example. Add your usage here if desired.");

    let dt = chrono::Utc::now();
    let dec = decimal_time::DecimalTime::from_datetime_utc(dt);
    println!("Now in Decimal Time = {:?}", dec);
}
