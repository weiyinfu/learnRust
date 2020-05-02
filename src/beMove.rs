#[cfg(test)]
mod beMove {
    use std::borrow::Borrow;

    //Rust天生移动语义
    #[test]
    fn moveSematic() {
        //写错了的移动语义：两个地址并不相同
        struct Node(i32);
        fn addr(x: &Node) -> String {
            format!("{}", x as *const Node as usize)
        }
        fn get() -> Node {
            let x = Node(3);
            println!("address of x in function :{:?}", addr(&x));
            x
        }
        let x = get();
        println!("{}", x.0);
        println!("address of x:{:?}", addr(&x));
    }

    #[test]
    fn moveSematic2() {
        //写对了的移动语义
        struct Node(i32);
        fn addr(x: &Node) -> String {
            format!("{}", x as *const Node as usize)
        }
        fn get() -> &'static Node {
            //get成了Node工厂
            //此处如果写成Node就会报错
            let x = &Node(3);
            println!("address of x in function :{:?}", addr(&x));
            &x
        }
        let x = get();
        println!("{}", x.0);
        println!("address of x:{:?}", addr(&x));
        let y = get();
        println!("address of y:{}", addr(y));
    }
}