use std::fs::File;
use std::io::Read;
use std::io;

#[test]
fn useAssert() {
    let x = true;
    assert!(x);
    let x = 13;
    assert_eq!(x, 13);
}

#[test]
fn useUnreachable() {
    //如果到达了unreachable代码就会报错
    print!("x");
    unreachable!("haha")
}

#[test]
fn usePanic() {
    panic!("hello");
}

#[test]
fn useResult() {
    let x: Result<&str, i32> = Result::Ok("haha");
    let y = x.expect("baga");
    println!("{}", y);
}

#[test]
fn returnResult() {
    fn isEven(x: i32) -> Result<&'static str, &'static str> {
        if (x & 1) == 0 { Result::Ok("yes") } else { Result::Err("no") }
    }
    for i in 1..10 {
        println!("Is {} is even?", i);
        match isEven(i) {
            Ok(value) => {
                println!("{}", value)
            }
            Err(value) => {
                println!("{}", value)
            }
        }
    }
}

#[test]
fn simpleWrite() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
        //使用问号表示如果出现错误，直接返回错误，这是一个语法糖
        File::open("a.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
    let a = read_username_from_file();
    print!("{:?}", a);
}