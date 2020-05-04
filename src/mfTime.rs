extern crate chrono;

use self::chrono::Timelike;

#[test]
fn printTime() {
    let x = chrono::Local::now();
    println!("timestamp={}", x.timestamp());
    println!("timestamp nano={}", x.nanosecond());
    println!("timestamp_millis={}", x.timestamp_millis())
}