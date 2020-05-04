use std::collections::LinkedList;

#[test]
fn main() {
    let mut lst1 = LinkedList::new();
    for i in 1..10 {
        lst1.push_back(i);
    }
    let mut lst2 = LinkedList::new();
    for i in 10..20 {
        lst2.push_back(i);
    }
    println!("{:?}", lst1);
    println!("{:?}", lst2);
    lst1.append(&mut lst2);
    println!("{:?}", lst1);
    println!("{:?}", lst2);
    lst1.pop_front();
    println!("{:?}", lst1.pop_back().unwrap());
    println!("{:?}", lst1);
    lst1.push_front(198);
    println!("{:?}", lst1);
}
