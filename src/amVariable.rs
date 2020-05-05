#[cfg(test)]
mod AMVariable {
    #[test]
    fn testVariable() {
        //变量先声明
        let x: i32;
        let y = 4;
        if y < 5 {
            x = 7
        } else {
            x = 8;
        }
        println!("{}", x);
    }

    #[test]
    fn variableRedefine() {
        //变量重定义
        let x = 3;
        println!("{}", x);
        let x = "weiyinfu";
        println!("{}", x);
    }

    #[test]
    fn testStruct() {
        struct Node {
            name: String
        }
        //如果没有mut就会报错
        let mut x = Node { name: String::from("weiyinfu") };
        x.name = String::from("haha");
        println!("{}", x.name);
    }
}

#[test]
fn getPointer() {
    //获取指针，因为3是常量，所以二者指向同一个地址
    let x = &3;
    let y = &3;
    println!("{}", x as *const i32 as usize);
    println!("{}", y as *const i32 as usize);
}