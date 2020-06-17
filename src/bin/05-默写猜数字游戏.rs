use rand::Rng;
use std::io::{stdin, stdout, Write};

fn main() {
    let mut r = rand::thread_rng();
    let secret = r.gen_range(0, 100);
    loop {
        let mut line = String::new();
        print!("猜一个数字吧：");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();
        // line = line.replace("\n", "");
        line = String::from(line.trim());
        let guess = line.parse::<i32>().unwrap();
        if guess == secret {
            println!("你真牛逼");
            break;
        }
        if guess < secret {
            println!("太小啦");
        } else {
            println!("太大了");
        }
    }
}
