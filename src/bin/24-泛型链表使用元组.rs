use std::fmt::{Display, Debug};
use std::borrow::Borrow;

#[derive(Clone, Debug)]
struct No<T> {
    value: T,
    next: Box<Node<T>>,
}

#[derive(Clone, Debug)]
enum Node<T> {
    NONE,
    Value(No<T>),
}

struct LinkedList<T> {
    head: Node<T>
}

fn getNode<T>(value: T) -> Node<T> {
    let no = No { value, next: Box::new(Node::NONE) };
    Node::<T>::Value(no)
}

impl<T> LinkedList<T> where T: Display + Debug {
    fn append(&mut self, value: T) {
        if let Node::Value(no) = &mut self.head {
            //如何把下面三句话调通
            // let next = no.next;
            // let node = Node::Value(No { value, next });
            // no.next = Box::new(node);
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
    let mut x: LinkedList<i32> = LinkedList { head: getNode(0) };
    x.append(3);
    x.append(4);
    x.print();
}