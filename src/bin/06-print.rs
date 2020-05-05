use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
/*
此例意在说明：print!不会调用flush，如果想要立即打印，需要手动flush
*/
fn main() {
    print!("haha");
    stdout().write("one".as_ref()).unwrap();
    stdout().write("\rtwo".as_ref()).unwrap();
    stdout().flush().unwrap();
    sleep(Duration::from_secs(3));
    print!("\rdfd");
    sleep(Duration::from_secs(3));
    print!("\rwhy");
}
