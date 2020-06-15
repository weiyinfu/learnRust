#[cfg(test)]
mod beMove {
    use std::borrow::Borrow;

    struct Node(i32);

    fn addr(x: &Node) -> String {
        format!("{}", x as *const Node as usize)
    }

    //Rust天生移动语义
    #[test]
    fn returnValue() {
        //不是移动语义：对于结构体和基本类型，只是在栈上发生复制
        fn get() -> Node {
            let y = 0; //y在栈上，占4个字节
            let x = Node(3); //x也在栈上，与y紧凑排列
            println!("address of y in function :{:?}", &y as *const i32 as usize);
            println!("address of x in function :{:?}", addr(&x));
            x
        }
        let y = 0; //y在栈上
        let x = get(); //x也在栈上
        println!("{}", x.0);
        println!("address of y:{:?}", &y as *const i32 as usize);
        println!("address of x:{:?}", addr(&x));
    }

    #[test]
    fn returnPointer() {
        //不是移动语义：对于结构体和基本类型，只是在栈上发生复制
        //vec的指针始终在栈上，vec的指针还是会发生复制。
        fn get() -> Vec<i32> {
            let y = 0; //y在栈上，占4个字节
            let x = Vec::new(); //x也在栈上，与y紧凑排列，24B，pos，lens，capacity
            let z = 0;
            println!("address of y in function :{:?}", &y as *const i32 as usize);
            println!("address of z in function :{:?}", &z as *const i32 as usize);
            println!(
                "address of x in function :{:?}",
                &x as *const Vec<i32> as usize
            );
            x
        }
        let y = 0; //y在栈上
        let x = get(); //x也在栈上
        println!("address of y:{:?}", &y as *const i32 as usize);
        println!("address of x:{:?}", &x as *const Vec<i32> as usize);
    }

    #[test]
    fn moveSematic2() {
        //写对了的移动语义
        fn get() -> &'static Node {
            //get成了Node工厂
            //此处如果写成Node就会报错
            let y = 0; //y在栈上
            let x = &Node(3);
            println!("address of y in function :{:?}", &y as *const i32 as usize);
            println!("address of x in function :{:?}", addr(&x));
            &x
        }
        let z = 0; //y在栈上
        let x = get();
        println!("{}", x.0);
        println!("address of z :{:?}", &z as *const i32 as usize);
        println!("address of x:{:?}", addr(&x));
        let y = get();
        println!("address of y:{}", addr(&y));
    }
}

#[test]
fn changeMutabilityInMove() {
    //在移动过程中改变可变性
    let x = Box::new(3);
    let mut y = x;
    *y = 4;
    //x不能再访问了
    // println!("{}", *x);
    println!("{}", y);
}
