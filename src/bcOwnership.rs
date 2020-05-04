#[cfg(test)]
mod bcOwnership {
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
        let mut x = Node(3);
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
        let mut x = Node(3);
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
    fn dangling() {
        //悬空
        let a;
        {
            let b = 10;
            a = &b;
        }
        //下面这句话会报错
        // println!("a : {}", a);
    }
}