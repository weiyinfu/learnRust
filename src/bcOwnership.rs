#[cfg(test)]
#[allow(unused_variables)]
mod bcOwnership {
    #[test]
    fn one() {
        /*
            引用的规则：
            在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
        引用必须总是有效的。
                        */
        let mut s = String::from("hello");
        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        let r3 = &mut s;
        //下面这句话报错：r1和r2已经不能用了
        // println!("{}, {}, and {}", r1, r2, r3);
        // println!("{}", s);//s已经被借出去了，尚未归还
        println!("{}", r3);
    }

    #[test]
    fn two() {
        let mut s = String::from("hello");
        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        { //即便是放在一个作用域里面也有问题
             // let r3 = &mut s;
        }
        println!("{}, {}", r1, r2);
        println!("{}", s); //s已经被借出去了，尚未归还
    }

    #[test]
    fn basicTypeAssinment() {
        //基础类型的复制ownership不变，而是简单的值复制
        let mut x = 3;
        let y = x;
        println!("x={} y={}", x, y);
        x = 100;
        println!("x={} y={}", x, y);
    }

    #[test]
    fn variableAssinment() {
        struct Node(i32);
        let x = Node(3);
        let y = x;
        //下面的语句会报错，因为x的所有权已经交给y了
        // println!("{}", x.0);
    }

    #[test]
    fn copyableVariableAssinment() {
        /*
        在上面的例子中，基本数据类型都实现了Copy和Clone两个接口，所以发生传值
        在第二个例子中，Node没有Copy和Clone，所以赋值操作发生所有权转移
        */
        struct Node(i32);
        impl Copy for Node {}
        impl Clone for Node {
            fn clone(&self) -> Self {
                Node(self.0)
            }

            fn clone_from(&mut self, source: &Self) {
                self.0 = source.0;
            }
        }
        let x = Node(3);
        let y = x;
        println!("{}", x.0);
    }

    #[test]
    fn heapVariable() {
        //x是一个指针，它指向堆内存，只要有堆内存，那么赋值操作就会发生move，发生所有权转移。
        let x = Box::new(3);
        let y = x;
        //报错：x的值已经借出去了
        // println!("x={} y={}", x, y)
    }

    #[test]
    #[allow(unused_assignments)]
    fn dangling() {
        //悬空
        let a;
        {
            let b = 10;
            a = &b;
        }
        //下面这句话会报错，因为a指向的空间已经被释放了
        // println!("a : {}", a);
    }
}

#[test]
fn freeze() {
    //如果把读引用借用出去了，那么就禁止对原来的数据进行修改
    let mut _mutable_integer = 7i32;

    {
        // 借用 `_mutable_integer`
        let large_integer = &_mutable_integer;

        // 报错！`_mutable_integer` 在本作用域被冻结
        // _mutable_integer = 50;
        // 改正 ^ 注释掉此行

        println!("Immutably borrowed {}", large_integer);

        // `large_integer` 离开作用域
    }

    // 正常运行！`_mutable_integer` 在这作用域没有冻结
    _mutable_integer = 3;
}

#[test]
fn refKeyword() {
    //ref关键字,ref关键字只是写起来比较美观，它完全是可有可无的东西
    let mut x = 3;
    let ref y = x;
    let z = &x;
    let ref mut a = x;
    // let mut ref rea = x;
}
