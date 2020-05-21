/*
理解了生命周期就能够理解很多东西：
https://weiyinfu.cn/RustBook/ch10-03-lifetime-syntax.html
*/
#[test]
fn one() {
    struct Example<'a> {
        x: &'a i32,
    }
    let y = &9;
    let b = Example { x: y };
    println!("{}", b.x);
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
    let a1 = Example { a: 10 };
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
    //不加生命周期会报错
    // fn longest(x: &str, y: &str) -> &str {
    //     //‘a取x和y中生命周期较短的那一个，生命周期是Rust给每个引用添加的一个属性
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
    // let x = "one";
    // {
    //     let y = "two";
    //     println!("{}", longest(x, y));
    // }
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
