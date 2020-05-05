/*
Some,Optional,Result等常用数据结构
*/

use std::fs::File;

#[test]
#[allow(unused_variables)]
fn optionAndSome() {
    let x: Option<i32> = Some(10);  // 'T' is of type i32.
    let x: Option<bool> = Some(true);  // 'T' is of type bool.
    let x: Option<f64> = Some(10.5); // 'T' is of type f64.
    let x: Option<char> = Some('b'); // 'T' is of type char.
    println!("{}", x.unwrap());
}

#[test]
fn useSome() {
    fn even_number(n: i32) {
        let num = n;
        match checked_even(n) {
            None => println!("None"),
            Some(n) => {
                if n == 0 {
                    println!("{} is a even number", num);
                } else {
                    println!("{} is a odd number", num);
                }
            }
        }
    }
    fn checked_even(number: i32) -> Option<i32> {
        Some(number % 2)
    }
    even_number(2);
    even_number(3);
}

#[test]
fn useSome2() {
    let a: Option<i32> = Some(5);
    match a {
        Some(5) => println!("five"),
        _ => (),
    }
    let a = Some(3);
    if let Some(3) = a {
        println!("three");
    }
}

#[test]
fn useResult() {
    let f = File::open("Cargo.toml");
    match f {
        Ok(file) => {
            println!("打开文件成功");
            file
        }
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
}

#[test]
fn useResult2() {
    enum Res<T, E> {
        OK(T),
        Err(E),
    }
    let x: Res<&'static str, i32> = Res::OK("haha");
    match x {
        Res::OK(value) => {
            println!("{}", value);
        }
        Res::Err(value) => {
            println!("{}", value);
        }
    }
}