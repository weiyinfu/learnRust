// use rand;
// use rand::{ThreadRng, Rng};
//
// #[derive(Debug)]
// enum Node {
//     null,
//     Node {
//         value: i32,
//         left: Box<Node>,
//         right: Box<Node>,
//     },
// }
//
// fn generateTree(r: &mut ThreadRng) -> &'static Node {
//     let x = &Node::Node { value: 12, left:Box (Node::null), right: Node::null };
//     let mut values = Vec::new();
//     for i in 1..10 {
//         values.push(r.gen_range(0, 100));
//     }
//     fn insert(root: &mut Node, value: i32) {
// //向二叉树中插入结点
//         if root == Node::null {
//             // root = Node::Node { value: value, left: Node::null, right: Node::null };
//             return;
//         }
//         if value > root.value {
//             insert(&mut root.right, value);
//         } else if value < root.value {}
//     }
//     return x;
// }
//
// fn main() {
//     let mut r = rand::thread_rng();
//     let tree = generateTree(&mut r);
//     println!("{:?}", tree);
// }