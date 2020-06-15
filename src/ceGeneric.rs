/*
使用泛型
*/
use std::fmt::{Debug, Display};

use rand::random;

#[test]
fn genericFunction() {
    //插入排序
    fn insertSort<T: PartialOrd + Debug + Copy>(a: &mut Vec<T>) {
        for i in 0..a.len() {
            for j in i + 1..a.len() {
                if a[j] < a[i] {
                    let temp = a[i];
                    a[i] = a[j];
                    a[j] = temp;
                }
            }
        }
    }
    let mut a: Vec<i32> = Vec::new();
    for _i in 1..10 {
        a.push(random());
    }
    insertSort(&mut a);
    //显式指明类型
    insertSort::<i32>(&mut a);
    println!("{:?}", a);
}

#[test]
fn genericStruct() {
    //泛型结构体
    #[derive(Debug)]
    struct Node<T> {
        name: T,
    }
    let x = Node { name: "weiyinfu" };
    let y = Node { name: 18 };
    println!("{:?} {:?}", x, y);
}

#[test]
fn genericEnum() {
    #[derive(Debug)]
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        OK(T),
        Err(E),
    }
}

#[test]
fn genericClass() {
    struct Node<T: Debug + Display> {
        name: T,
    }
    impl<T: Debug + Display> Node<T> {
        fn printName(self) {
            println!("{}", self.name)
        }
    }
    let x = Node { name: "weiyinfu" };
    x.printName();
    let x = Node { name: 18 };
    x.printName();
}

#[test]
#[allow(unused_variables)]
fn implPart() {
    //只为一部分泛型实现函数
    struct Node<T: Debug + Display> {
        name: T,
    }
    impl Node<&str> {
        fn printName(self) {
            println!("{}", self.name)
        }
    }
    let x = Node { name: "weiyinfu" };
    x.printName();
    let x = Node { name: 18 };
    //int类型的Node并没有printName函数
    // x.printName();
}

#[test]
fn genericAddress() {
    //使用泛型函数获取元素的内存地址
    #[allow(dead_code)]
    fn addr<T>(x: &T) -> usize {
        return x as *const T as usize;
    }

    let x = 0;
    let a = Vec::<i32>::new();
    let y = 0;
    println!("x={} {}", addr(&x), &x as *const i32 as usize);
    println!("y={} {}", addr(&y), &y as *const i32 as usize);
    println!("a={} {}", addr(&a), &a as *const Vec<i32> as usize);
}

#[test]
fn emptyConstriction() {
    //使用空约束
    trait Two {}
    fn haha<T: Two + Display>(x: T) {
        println!("{}", x)
    }
    impl Two for i32 {}
    haha(3);
}

#[test]
fn useWhere() {
    //where比<T:Display>这种表达形式表现力更强，此例只能使用where而无法使用尖括号形式实现
    use std::fmt::Debug;

    trait PrintInOption {
        fn print_in_option(self);
    }

    // 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`（这样意思就变了），
    // 或着改用另一种间接的方法。
    impl<T> PrintInOption for T
    where
        Option<T>: Debug,
    {
        // 我们要将 `Option<T>: Debug` 作为约束，因为那是要打印的内容。
        // 否则我们会给出错误的约束。
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}

#[test]
#[allow(unused_variables)]
fn useAssociateTypes() {
    /*
    关联类型的意义：减少泛型参数
    */
    //泛型trait的两种方式：不使用关联类型、使用关联类型
    //使用关联类型
    mod one {
        // use generic parameters
        pub trait Graph<N, E> {
            fn has_edge(&self, startNode: &N, endNode: &N) -> bool;
            fn edges(&self, node: &N) -> Vec<E>;
        }

        fn distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {
            0
        }
    }
    mod two {
        // use associated types
        pub(crate) trait Graph {
            type N;
            type E;

            fn has_edge(&self, start: &Self::N, end: &Self::N) -> bool;
            fn edges(&self, node: &Self::N) -> Vec<Self::E>;
        }

        fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) -> u32 {
            0
        }
    }
    use two::*;
    struct Node;
    struct Edge;
    struct SimpleGraph;
    impl Graph for SimpleGraph {
        type N = Node;
        type E = Edge;
        fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
            false
        }
        fn edges(&self, n: &Node) -> Vec<Edge> {
            Vec::new()
        }
    }
    let graph = SimpleGraph;
    let object = Box::new(graph) as Box<dyn Graph<N = Node, E = Edge>>;
}

#[test]
fn usePhatomData() {
    use std::marker::PhantomData;

    // 这个虚元组结构体对 `A` 是泛型的，并且带有隐藏参数 `B`。
    #[derive(PartialEq)] // 允许这种类型进行相等测试（equality test）。
    struct PhantomTuple<A, B>(A, PhantomData<B>);

    // 这个虚类型结构体对 `A` 是泛型的，并且带有隐藏参数 `B`。
    #[derive(PartialEq)] // 允许这种类型进行相等测试。
    struct PhantomStruct<A, B> {
        first: A,
        phantom: PhantomData<B>,
    }

    // 注意：对于泛型 `A` 会分配存储空间，但 `B` 不会。
    //       因此，`B` 不能参与运算。

    // 这里的 `f32` 和 `f64` 是隐藏参数。
    // 被指定为 `<char, f32>` 的 `PhantomTuple` 类型。
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // 被指定为 `<char, f64>` `PhantomTuple` 类型。
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // 被指定为 `<char, f32>` 的类型。
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // 被指定为 `<char, f64>` 的类型。
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // 编译期错误！类型不匹配，所以这些值不能够比较：
    //println!("_tuple1 == _tuple2 yields: {}",
    //          _tuple1 == _tuple2);

    // 编译期错误！类型不匹配，所以这些值不能够比较：
    //println!("_struct1 == _struct2 yields: {}",
    //          _struct1 == _struct2);
}
