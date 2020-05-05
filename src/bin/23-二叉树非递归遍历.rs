#![allow(non_snake_case, unused_mut, unused_variables)]

use rand;
use rand::Rng;

#[derive(Debug)]
enum Node {
    NULL,
    Node(No),
}

#[derive(Debug)]
struct No {
    value: i32,
    left: Box<Node>,
    right: Box<Node>,
}

fn get(value: i32) -> Node {
    return Node::Node(No {
        value,
        left: Box::new(Node::NULL),
        right: Box::new(Node::NULL),
    });
}

fn generateTree() -> Node {
    let mut values = Vec::new();
    let mut r = rand::thread_rng();
    for i in 1..10 {
        values.push(r.gen_range(0, 100));
    }
    fn insert(root: &mut Node, value: i32) {
        //向二叉树中插入结点
        if let Node::NULL = root {
            *root = get(value);
            return;
        }
        if let Node::Node(no) = root {
            if no.value > value {
                insert(&mut *no.right, value);
            } else {
                insert(&mut *no.left, value);
            }
        }
    }
    let mut root = get(values[0]);
    for i in 1..values.len() {
        insert(&mut root, values[i]);
    }
    root
}

fn preOrder(root: &Node) {
    //先序遍历
    let mut sta = vec![&*root];
    while !sta.is_empty() {
        let now = sta.pop().unwrap();
        if let Node::Node(no) = now {
            print!("{},", no.value);
            if let Node::NULL = *no.right {
            } else {
                sta.push(&*no.right);
            }
            if let Node::NULL = *no.left {
            } else {
                sta.push(&*no.left);
            }
        }
    }
    println!()
}

fn printTree(node: &Node, pad: i32) {
    if let Node::Node(no) = node {
        for i in 0..pad {
            print!(" ")
        }
        println!("{}", no.value);
        printTree(&*no.left, pad + 1);
        printTree(&*no.right, pad + 1);
    }
}

fn midOrder(node: &Node) {
    let mut sta = vec![(node, false)];
    while !sta.is_empty() {
        let (now, visited) = sta.pop().unwrap();
        if let Node::Node(ref no) = now {
            if visited {
                print!("{},", no.value);
            } else {
                if let Node::Node(r) = &*no.right {
                    sta.push((&*no.right, false));
                }
                sta.push((now, true));
                if let Node::Node(l) = &*no.left {
                    sta.push((&*no.left, false));
                }
            }
        }
    }
    println!()
}

fn backOrder(node: &Node) {
    let mut sta = vec![(node, false)];
    while !sta.is_empty() {
        let (now, visited) = sta.pop().unwrap();
        if let Node::Node(ref no) = now {
            if visited {
                print!("{},", no.value);
            } else {
                sta.push((now, true));
                if let Node::Node(r) = &*no.right {
                    sta.push((&*no.right, false));
                }
                if let Node::Node(l) = &*no.left {
                    sta.push((&*no.left, false));
                }
            }
        }
    }
    println!()
}

fn main() {
    let mut tree = generateTree();
    printTree(&tree, 0);
    preOrder(&tree);
    midOrder(&tree);
    backOrder(&tree);
}
