/*
使用泛型
*/
use rand::random;
use std::fmt::{Debug, Display};

#[test]
fn genericFunction() {
    //插入排序
    fn insertSort<T: PartialOrd + Debug + Copy>(a: &mut Vec<T>) {
        for i in 0..a.len() {
            for j in i + 1..a.len() {
                if a[j] < a[i] {
                    let temp = a[i];
                    a[i] = a[j];
                    a[j] = temp;
                }
            }
        }
    }
    let mut a: Vec<i32> = Vec::new();
    for _i in 1..10 {
        a.push(random());
    }
    insertSort(&mut a);
    println!("{:?}", a);
}

#[test]
fn genericStruct() {
    //泛型结构体
    #[derive(Debug)]
    struct Node<T> {
        name: T
    }
    let x = Node { name: "weiyinfu" };
    let y = Node { name: 18 };
    println!("{:?} {:?}", x, y);
}

#[test]
fn genericEnum() {
    #[derive(Debug)]
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E>
    {
        OK(T),
        Err(E),
    }
}

#[test]
fn genericClass() {
    struct Node<T: Debug + Display> {
        name: T
    }
    impl<T: Debug + Display> Node<T> {
        fn printName(self) {
            println!("{}", self.name)
        }
    }
    let x = Node { name: "weiyinfu" };
    x.printName();
    let x = Node { name: 18 };
    x.printName();
}

#[test]
#[allow(unused_variables)]
fn implPart() {
    //只为一部分泛型实现函数
    struct Node<T: Debug + Display> {
        name: T
    }
    impl Node<&str> {
        fn printName(self) {
            println!("{}", self.name)
        }
    }
    let x = Node { name: "weiyinfu" };
    x.printName();
    let x = Node { name: 18 };
    //int类型的Node并没有printName函数
    // x.printName();
}