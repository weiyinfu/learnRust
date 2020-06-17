#![allow(non_snake_case, unused_variables, dead_code)]

use std::fmt::{Debug, Display};
use std::sync::Arc;

enum Node<T> {
    NONE,
    Value { value: T, next: Arc<Node<T>> },
}

struct LinkedList<T> {
    head: Node<T>,
}

fn getNode<T>(value: T) -> Node<T> {
    return Node::Value {
        value,
        next: Arc::new(Node::NONE),
    };
}

impl<T> LinkedList<T>
where
    T: Display + Debug,
{
    fn append(&mut self, value: T) {
        /*
        使用结构体的坏处是，解构结构体的时候需要写很多
        */
        if let Node::Value { next, .. } = &mut self.head {
            //如何把下面这句话调通
            let temp = (*next).to_owned();
            let now = Node::Value { value, next: temp };
            *next = Arc::new(now);
        } else {
            unreachable!()
        }
    }
    fn remove(&self, value: T) {}
    fn indexOf(&self, value: T) {}
    fn print(&self) {
        let mut now = &self.head;
        loop {
            if let Node::Value { value, next } = now {
                print!("{}->", value);
                now = &*next;
            } else {
                break;
            }
        }
    }
}

fn main() {
    let mut x: LinkedList<i32> = LinkedList { head: getNode(0) };
    x.append(3);
    x.append(4);
    x.print();
}
