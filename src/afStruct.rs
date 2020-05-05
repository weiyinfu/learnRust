#[cfg(test)]
mod afStruct {
    #[test]
    fn structMethods() {
        //为struct添加方法
        struct Triangle {
            a: f64,
            b: f64,
            c: f64,
        }
        impl Triangle {
            fn area(&self) -> f64 {
                let p = (self.a + self.b + self.c) / 2.0;
                return (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt();
            }
        }
        let x = Triangle { a: 2f64, b: 2f64, c: 3f64 };
        println!("{}", x.area());
    }
}

#[test]
fn deconstruct() {
    struct Node {
        name: String,
        age: i32,
    }
    let x = Node { name: String::from("hha"), age: 112 };
    //使用..忽略其它字段
    let Node { age: AGE, .. } = x;
    println!("{}", AGE);
}

#[test]
fn threeFunction() {
    struct Node {}
    //结构的三种函数
    impl Node {
        fn one() {
            //静态函数
        }
        fn two(&self) {
            //不可变引用，只读
        }
        fn three(self) {
            //move引用，只能调用一次
        }
        fn four(&mut self) {
            //可写引用，要想调用这个函数，self必须可写
        }
    }
    let x = Node {};
    x.three();
    //再次执行下面语句就会报错
    // x.three();
}