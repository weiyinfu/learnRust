#![allow(non_snake_case)]

use chrono::Local;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use rand::{thread_rng, Rng};
/*
手写进度条，类似Python中的Tqdm
*/
pub struct Progress {
    lastPrint: i64,
    begTime: i64,
    total: usize,
    now: usize,
    desc: String,
}

fn getTime() -> i64 { Local::now().timestamp_millis() }

fn formatMilliSeconds(milli: i64) -> String {
    let mut seconds = milli / 1000;
    let day = seconds / (24 * 3600);
    seconds %= 24 * 3600;
    let hour = seconds / 3600;
    seconds %= 3600;
    let minute = seconds / 60;
    seconds %= 60;
    let mut s = String::new();
    if day != 0 {
        s.push_str(&format!("{}天", day));
    }
    if hour != 0 {
        s.push_str(&format!("{}小时", hour));
    }
    if minute != 0 {
        s.push_str(&format!("{}分", minute));
    }
    if seconds != 0 {
        s.push_str(&format!("{}秒", seconds));
    }
    s
}

impl Progress {
    pub(crate) fn new(total: usize, desc: String) -> Progress {
        Progress { lastPrint: getTime(), total, now: 0, begTime: -1, desc }
    }
    pub(crate) fn update(&mut self, cnt: i32) {
        let now = getTime();
        if self.begTime == -1 {
            self.begTime = now;
        }
        self.now += cnt as usize;
        if now - self.lastPrint > 1000 && self.now > 2 {
            //如果打印时间大于1s，则打印新消息
            self.lastPrint = now;
            let used = now - self.begTime;//已花费的时间
            let need: i64 = ((used as f64) / (self.now as f64) * ((self.total - self.now) as f64)) as i64;//还需要的时间
            let percent = ((self.now as f64 / self.total as f64) * 100.0) as i32;
            let speed = self.now as i64 * 1000 / used;
            let needString = formatMilliSeconds(need);
            let usedString = formatMilliSeconds(used);
            print!("\r【{}】 进度={}% {}/{} {}iter/s 已用{},还需{}",
                   self.desc,
                   percent,
                   self.now, self.total,
                   speed,
                   usedString,
                   needString);
            stdout().flush().unwrap();
        }
    }
}

fn main() {
    let mut x = Progress::new(10, "迭代".to_string());
    let mut rng = thread_rng();
    for _i in 1..10 {
        sleep(Duration::from_secs(rng.gen_range(1, 3)));
        x.update(1);
    }
}