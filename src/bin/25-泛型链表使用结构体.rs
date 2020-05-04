use std::fmt::Display;

enum Node {
    NONE,
    Value {
        value: i32,
        next: Box<Node>,
    },
}

struct LinkedList {
    head: Node
}

fn getNode(value: i32) -> Node {
    return Node::Value { value, next: Box::new(Node::NONE) };
}

impl LinkedList {
    fn append(&mut self, value: i32) {
        /*
        使用结构体的坏处是，解构结构体的时候需要写很多
        */
        if let Node::Value { value, next } = &mut self.head {
            //如何把下面这句话调通
            let now = Node::Value { value: *value, next: *next };
            *next = Box::new(now);
        } else {
            unreachable!()
        }
    }
    fn remove(&self, value: i32) {}
    fn indexOf(&self, value: i32) {}
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
    let mut x: LinkedList = LinkedList { head: getNode(0) };
    x.append(3);
    x.append(4);
    x.print();
}