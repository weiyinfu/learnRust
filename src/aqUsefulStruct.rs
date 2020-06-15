/*
Some,Optional,Result等常用结构体
Optional是一个枚举，它包括Some和None，其中Some是一个元组结构体，None是一个空结构体
*/

use std::fs::File;

#[test]
#[allow(unused_variables)]
fn optionAndSome() {
    let x: Option<i32> = Some(10); // 'T' is of type i32.
    let x: Option<bool> = Some(true); // 'T' is of type bool.
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
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
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

#[test]
fn use_take() {
    let mut head = Some(Box::new(3));
    let mut p = &mut head;
    let x = p.take();
    println!("{}", head.is_none());
}

#[test]
fn cannot_use_unwrap() {
    let mut head = Some(Box::new(3));
    let mut p = &mut head;
    // let x = p.unwrap();
    //unwrap的参数是self，一定会发生move
}

#[test]
fn use_box() {
    use std::ops::{Deref, DerefMut};
    let mut x = Box::new(1);
    let mut y = x.to_owned();
    *y = 3;
    println!("{}", *x);
    let x = Some(Box::new(1));
    let mut y = x.to_owned();
    let z = y.as_deref_mut().unwrap();
    *z = 10;
    println!("{}", x.unwrap());
}
