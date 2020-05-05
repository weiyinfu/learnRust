/*
Box <T>：Box <T>是一个智能指针，指向在类型为T的堆上分配的数据，其中“T”是数据的类型。
它用于将数据存储在堆上而不是堆栈上。

Deref <T>：Deref <T>是一个智能指针，用于自定义解除引用运算符(*)的行为。
Drop <T>：Drop <T>是一个智能指针，用于在变量超出范围时从堆内存中释放空间。
Rc <T>：Rc <T>代表参考计数指针。它是一个智能指针，用于记录存储在堆上的值的引用数。
RefCell <T>：RefCell <T>是一个智能指针，允许借用可变数据，即使数据是不可变的。这个过程被称为内部可变性。
*/

#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};
use std::ops::Deref;

#[test]
fn link() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    print!("{:?}", list);
}

#[test]
fn useBox() {
    //理解box的原理，deref相当于*运算符
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn example(y: T) -> MyBox<T>
        {
            MyBox(y)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }
    let a = 8;
    let b = MyBox::example(a);
    print!("Value of *b is {}", *b);
}

