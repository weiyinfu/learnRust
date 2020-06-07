use std::ops::Add;

//Rust实现运算符重载
#[test]
fn useAdd() {
    #[derive(Debug)]
    struct Node {
        x: i32,
    }
    impl Add for Node {
        type Output = Node;

        fn add(self, rhs: Self) -> Self::Output {
            Self { x: self.x + rhs.x }
        }
    }
    let one = Node { x: 1 };
    let two = Node { x: 2 };
    println!("{:?}", one + two);
}
