#[test]
fn NestingModule() {
    mod a {
        pub mod b {
            pub struct Node {
                pub name: String,
            }
        }
    }
    let x = a::b::Node {
        name: String::from("haha"),
    };
    println!("{:?}", x.name);
}

#[test]
fn NestingModule2() {
    //使用包级别的可见性
    mod a {
        pub(crate) mod b {
            #[derive(Debug)]
            pub(crate) struct Node {
                pub(crate) name: String,
            }
        }
    }
    let x = a::b::Node {
        name: String::from("haha"),
    };
    println!("{:?}", x);
}

#[test]
fn testTupleStruct() {
    mod a {
        #[derive(Debug)]
        pub struct int(pub i32);

        #[derive(Debug)]
        pub struct int2(pub i32, i32);

        //下面这个结构体无法在外部进行初始化
        #[derive(Debug)]
        pub struct intPrivate(i32);
    }

    println!("{:?}", a::int(128));
    //下面两句话报错，包含不可见元素的Tuple结构体无法进行初始化
    // println!("{:?}", a::int2(128, 1));
    // println!("{:?}", a::intPrivate(128));
}

#[test]
#[allow(unused_assignments, unused_variables)]
fn useUse() {
    //使用Use可以减少
    mod a {
        pub mod b {
            pub struct Node {
                pub name: String,
            }
        }
    }
    use a::b as haha;
    let x = haha::Node {
        name: String::from("haha"),
    };
    println!("{}", x.name);
    use a::b::Node;
    let y = Node {
        name: String::from("ha"),
    };
    println!("{}", y.name);
    use a::b::Node as No;
    let z = No {
        name: String::from("no"),
    };
    println!("{}", z.name);
    use a::b::*; //引入全部
    let m = Node {
        name: String::from("haha"),
    };
}

#[test]
fn visitParentModule() {
    //访问父模块
    mod a {
        fn haha() {
            println!("haha")
        }

        mod c {
            pub(crate) fn haha() {
                println!("c haha");
            }
        }

        mod b {
            fn haha() {
                super::haha();
                super::c::haha();
            }
        }
    }
}

#[test]
fn reExport() {
    mod main {
        mod a {
            pub fn f() {}
        }

        pub use a::f;
    }
    main::f();
}

#[test]
fn useFolderModule() {
    use crate::useFolder;
    useFolder::baga();
}
