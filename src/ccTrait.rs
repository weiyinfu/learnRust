/*
traits的本质就是interface
*/
use rand::random;
use serde::export::fmt::Error;
use serde::export::Formatter;
use std::f64::consts::PI;
use std::fmt;
use std::fmt::Debug;
use std::io::Read;
use std::path::Display;

trait Area {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return PI * self.radius * self.radius;
    }
}

#[test]
fn first() {
    fn printArea(shape: impl Area) {
        println!("{}", shape.area());
    }
    printArea(Circle { radius: 2.0f64 });
    printArea(Rectangle {
        width: 1.2f64,
        height: 1.3f64,
    });
}

#[test]
fn second() {
    //Rust的多态
    let a: Vec<Box<dyn Area>> = vec![
        Box::new(Circle { radius: 2.0f64 }),
        Box::new(Rectangle {
            width: 1.2f64,
            height: 1.3f64,
        }),
    ];
    for i in a.iter() {
        println!("{}", i.area());
    }
}

#[test]
fn traitAsReturnValue() {
    //在Rust中一个函数只能返回一种类型，虽然可以返回一个接口，但是不能返回不同的类型，Rust不支持多态
    // fn randomShape() -> impl Area {
    //     if random() < 0.5 {
    //         return Circle { radius: random() };
    //     } else {
    //         return Rectangle { width: random(), height: random() };
    //     }
    // }
    // printArea(randomShape());
    // printArea(randomShape());
}

#[test]
fn traitAsReturnValue2() {
    fn getCircle(x: f64) -> impl Area {
        Circle { radius: x }
    }
    println!("{}", getCircle(3.0).area());
}

#[test]
fn useGenericFunction() {
    //在泛型参数中约束泛型
    fn printArea<T: Area>(shape: T) {
        println!("{}", shape.area());
    }
    printArea(Circle { radius: 2.0f64 });
    printArea(Rectangle {
        width: 1.2f64,
        height: 1.3f64,
    });
}

#[test]
fn useGenericFunction2() {
    //使用where子句约束泛型
    fn printArea<T>(shape: T)
    where
        T: Debug + Area,
    {
        println!("{}", shape.area());
    }
    printArea(Circle { radius: 2.0f64 });
    printArea(Rectangle {
        width: 1.2f64,
        height: 1.3f64,
    });
}

#[test]
fn genericClass() {
    struct Node<T: Area> {
        shape: T,
    }
    impl<T: Area> Node<T> {
        fn print(&self) {
            println!("{}", self.shape.area());
        }
    }
    let x = Node {
        shape: Circle { radius: 1.0 },
    };
    x.print();
}

#[test]
fn multiTrait() {
    trait Length {
        fn length(&self) -> f64;
    }
    impl Length for Circle {
        fn length(&self) -> f64 {
            2.0 * PI * self.radius
        }
    }
    impl Length for Rectangle {
        fn length(&self) -> f64 {
            2.0 * (self.width + self.height)
        }
    }
    fn print<T: Length + Area>(x: T) {
        println!("周长为：{},面积为:{}", x.length(), x.area());
    }
    print(Circle { radius: 10.0 });
}

#[test]
fn defaultMethod() {
    //接口可以有默认函数
    trait Length {
        fn length(&self) {
            println!("长度")
        }
    }
    impl Length for Circle {}
    impl Length for Rectangle {}
    let x = Circle { radius: 10.0 };
    x.length()
}

#[test]
fn extendTrait() {
    trait A {
        fn f(&self);
    }
    trait B: A {
        fn t(&self);
    }
    struct Example {
        first: String,
        second: String,
    }
    //Example在实现B之前，必须实现A，否则编译报错
    impl A for Example {
        fn f(&self) {
            print!("{} ", self.first);
        }
    }
    impl B for Example {
        fn t(&self) {
            print!("{}", self.second);
        }
    }
    let x = Example {
        first: String::from("one"),
        second: String::from("two"),
    };
    x.t();
    x.f();
}

#[test]
fn twoTrait() {
    trait A {
        fn a(&self);
    }
    trait B {
        fn b(&self);
        fn a(&self);
    }
    fn go(x: &impl A) {
        x.a();
    }
    struct One {}
    impl B for One {
        fn b(&self) {
            println!("B.b")
        }

        fn a(&self) {
            println!("B.a")
        }
    }
    let x = One {};
    //这句话报错，虽然B的函数包括A，但是One结构体依旧需要实现A
    // go(x);
    impl A for One {
        fn a(&self) {
            println!("A.a");
        }
    }
    go(&x);
    A::a(&x);
    B::a(&x);
}

#[test]
fn twoTrait2() {
    struct Test;

    trait Trait1 {
        fn foo();
    }

    trait Trait2 {
        fn foo();
    }

    impl Trait1 for Test {
        fn foo() {}
    }
    impl Trait2 for Test {
        fn foo() {}
    }

    <Test as Trait1>::foo()
}

#[test]
fn forbidden() {
    // 不能为外部类型实现外部 trait，这被称为“孤儿规则”，这个限制被称为“相干性”，这是为了避免当前的实现干扰外部实现。
    // impl fmt::Display for Vec<i32> {
    //     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    //         write!(f, "nothing")
    //     }
    // }
    let x = vec![1, 2, 3];
    // println!("{}", x);
}
