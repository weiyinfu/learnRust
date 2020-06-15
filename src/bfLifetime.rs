/*
理解了生命周期就能够理解很多东西：
https://weiyinfu.cn/RustBook/ch10-03-lifetime-syntax.html
*/
#[test]
fn structLifetime() {
    struct Example<'a> {
        x: &'a i32,
    }
    let y = &9;
    let b = Example { x: y };
    println!("{}", b.x);
}

#[test]
fn variableScope() {
    let mut x = 3;
    {
        let x = 4;
    }
    println!("{}", x); //output 3
}

#[test]
fn second() {
    struct Example<'a> {
        x: &'a i32,
    }
    impl<'a> Example<'a> {
        fn display(&self) {
            print!("Value of x is : {}", self.x);
        }
    }
    let y = &90;
    let b = Example { x: y };
    b.display();
}

#[test]
#[allow(unused_variables)]
fn dropTrait() {
    struct Example {
        a: i32,
    }
    impl Drop for Example {
        fn drop(&mut self) {
            println!("Dropping the instance of Example with data : {}", self.a);
        }
    }
    {
        let a1 = Example { a: 10 };
    }
    println!("========");
    let b1 = Example { a: 20 };
    println!("Instances of Example type are created");
}

#[test]
#[allow(unused_variables)]
fn handDrop() {
    //手动drop，不能调用a.drop()函数，而应该调用mem中的drop函数
    struct Example {
        a: i32,
    }
    impl Drop for Example {
        fn drop(&mut self) {
            println!("Dropping the instance of Example with data : {}", self.a);
        }
    }
    let a1 = Example { a: 10 };
    let b1 = Example { a: 20 };
    //不允许主动drop
    // a1.drop();
    std::mem::drop(a1);
    println!("Instances of Example type are created");
}

#[test]
fn longestDemo() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        //‘a取x和y中生命周期较短的那一个，生命周期是Rust给每个引用添加的一个属性
        //此处如果不加生命周期，默认生命周期各不相同
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let x = "one";
    {
        let y = "two";
        println!("{}", longest(x, y));
    }
}

#[test]
fn longestDemo2() {
    // 在有返回值的情况下，不加生命周期会报错
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
}

#[test]
fn longestDemo3() {
    // 在有返回值的情况下，如果只有一个参数，不加生命周期不会报错。如果有多个应用，并且有返回值，那么必须带上生命周期参数
    fn longest(x: &str) -> &str {
        x
    }
}

#[test]
fn parentDependentSon() {
    //结构体依赖于它的成员变量而存在
    #[derive(Debug)]
    struct Node<'a> {
        name: &'a String,
    }
    fn getNode(name: &String) -> Node {
        //这种写法是省略生命周期的写法
        return Node { name };
    }
    let name = "haha".to_string();
    //当name消亡的时候，x也会消亡
    let x = getNode(&name);
    println!("{:?}", x);
}

#[test]
fn parentDependentSon2() {
    //结构体依赖于它的成员变量而存在
    #[derive(Debug)]
    struct Node<'a, 'b> {
        name: &'a String,
        addr: &'b String,
    }
    fn getNode<'a, 'b>(name: &'a String, addr: &'b String) -> Node<'a, 'b> {
        //这种写法是省略生命周期的写法
        //Node的生命周期会取‘a和’b中的较小值
        Node { name, addr }
    }
    let name = "haha".to_string();
    //当name消亡的时候，x也会消亡
    let x: Node;
    {
        let addr = "beijing".to_string();
        x = getNode(&name, &addr);
        println!("{:?}", x);
    }
    //下面这句话肯定报错，因为Node结构体依赖的addr已经把Node干掉了
    // println!("{:?}", x);
}

#[test]
fn parentDependentSonGeneric() {
    //结构体依赖于它的成员变量而存在
    #[derive(Debug)]
    struct Node<'a, 'b, Name, Addr> {
        name: &'a Name,
        addr: &'b Addr,
    }
    fn getNode<'a, 'b, Name, Addr>(name: &'a Name, addr: &'b Addr) -> Node<'a, 'b, Name, Addr> {
        //这种写法是省略生命周期的写法
        //Node的生命周期会取‘a和’b中的较小值
        Node { name, addr }
    }
    let name = "haha".to_string();
    //当name消亡的时候，x也会消亡
    let x: Node<String, String>; //声明的时候不用指定生命周期
    {
        let addr = "beijing".to_string();
        x = getNode(&name, &addr);
        println!("{:?}", x);
    }
    //下面这句话肯定报错，因为Node结构体依赖的addr已经把Node干掉了
    // println!("{:?}", x);
}

#[cfg(test)]
mod StrangeLoop {
    //使用生命周期的时候可能发生多次引用
    #[test]
    fn one() {
        let mut x = Box::new(3);
        let mut y = &mut x;
        fn f<'a>(a: &'a mut &'a mut Box<i32>) {}
        //下面两句话只能执行一次，因为只要调用f函数，那么f函数就一直没有归还
        // f(&mut y);
        f(&mut y);
    }

    #[test]
    fn two() {
        //在上例的基础上，把生命周期去掉就正确了，因为默认各个引用的生命周期各不相同
        let mut x = Box::new(3);
        let mut y = &mut x;
        fn f(a: &mut &mut Box<i32>) {}
        f(&mut y);
        f(&mut y);
    }

    #[test]
    fn three() {
        //如果执意带着生命周期，那也不是不可以。Rust默认每个参数生命周期符号都不相同，所以手动加上是画蛇添足。
        let mut x = Box::new(3);
        let mut y = &mut x;
        fn f<'a, 'b>(a: &'b mut &'a mut Box<i32>) {}
        //下面两句话只能执行一次，因为只要调用f函数，那么f函数就一直没有归还
        // f(&mut y);
        f(&mut y);
    }

    #[test]
    fn strangeLoop() {
        //上例中one的另一种表现形式
        let mut x = Box::new(3);
        let mut cur = &mut x;
        //go函数会自动取生命周期较长的那一个，所以此处go借用的生命周期与cur一致，而与currcur不一致
        fn go<'a>(curcur: &'a mut &'a mut Box<i32>) {}
        for i in 0..5 {
            let curcur = &mut cur;
            //取消注释下面这句话就会报错
            // go(curcur);
        }
    }
}

#[test]
fn useStruct() {
    struct A<'a> {
        name: &'a str,
    }

    impl<'a> A<'a> {
        //注意，此处返回值如果不加‘a注解，则返回的str与self生命周期相同
        fn get(&self) -> &'a str {
            self.name
        }
    }

    let s = String::from("hello");
    let s_ref;

    {
        let a = A { name: &s };
        s_ref = a.get();
    }
    println!("{:?}", s_ref);
}
