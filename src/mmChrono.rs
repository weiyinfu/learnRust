use std::ops::Add;
use chrono::{Duration, Utc, TimeZone};

#[test]
fn iterateDay() {
    //迭代日期
    let now = chrono::Date::from(Utc.ymd(1993, 10, 01));
    for i in 1..10 {
        let t = now.add(Duration::days(i));
        println!("{}", t.format("%Y-%m-%d"));
    }
    println!("{}", now.format("%Y-%m-%d"));
}