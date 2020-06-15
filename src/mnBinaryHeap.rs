use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[test]
fn haha() {
    //默认是大顶堆
    let mut q = BinaryHeap::<i32>::new();
    q.push(2);
    q.push(1);
    q.push(3);
    while !q.is_empty() {
        println!("{}", q.pop().unwrap());
    }
}
