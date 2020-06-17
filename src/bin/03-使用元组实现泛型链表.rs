#![allow(non_snake_case)]
use serde::export::Formatter;
use std::fmt;
use std::fmt::{Debug, Display};
use std::sync::Arc;

#[derive(Clone, Debug)]
struct No<T> {
    value: T,
    next: Arc<Node<T>>,
}

#[derive(Clone, Debug)]
enum Node<T> {
    NONE,
    Value(No<T>),
}

struct LinkedList<T> {
    head: Node<T>,
}

fn getNode<T>(value: T) -> Node<T> {
    let no = No {
        value,
        next: Arc::new(Node::NONE),
    };
    Node::<T>::Value(no)
}

impl<T> LinkedList<T>
where
    T: Display + Debug,
{
    fn append(&mut self, value: T) {
        if let Node::Value(no) = &mut self.head {
            let node = Node::Value(No {
                value,
                next: no.next.to_owned(),
            });
            no.next = Arc::new(node);
        } else {
            unreachable!()
        }
    }
    fn print(&self) {
        let mut now = &self.head;
        loop {
            if let Node::Value(no) = now {
                print!("{:?}->", no.value);
                now = &no.next;
            } else {
                break;
            }
        }
    }
}

fn main() {
    #[derive(Debug)]
    struct Student {
        name: String,
        age: i32,
    }
    impl Display for Student {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    let mut x: LinkedList<Student> = LinkedList {
        head: getNode(Student {
            name: String::from("haha"),
            age: 18,
        }),
    };
    x.append(Student {
        name: "second".to_string(),
        age: 2,
    });
    x.append(Student {
        name: "third".to_string(),
        age: 3,
    });
    x.print();
}
