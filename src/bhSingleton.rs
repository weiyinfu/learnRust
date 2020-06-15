#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[test]
fn WritableSingleton() {
    //可以执行写操作的单例
    use std::sync::Arc;
    use std::sync::Mutex;

    impl Point {
        pub fn get_instance() -> Arc<Mutex<Point>> {
            static mut POINT: Option<Arc<Mutex<Point>>> = None;

            unsafe {// Rust中使用可变静态变量都是unsafe的
                POINT.get_or_insert_with(|| {
                    // 初始化单例对象的代码
                    Arc::new(Mutex::new(Point { x: 0, y: 0 }))
                }).clone()
            }
        }
    }
    {
        let  x = Point::get_instance();
        let mut p = x.lock().unwrap();
        println!("{:?}", p);
        p.x = 13;
        println!("{:?}", p);
    }
    let  xx = Point::get_instance();
    println!("{:?}", xx.lock().unwrap());
}

#[test]
fn readableSingleton() {
    use std::sync::Arc;
    //获取只读的单例有两种写法：第一种是使用Arc<Point>
    //第二种是使用‘static
    fn get_instance1() -> Arc<Point> {
        static mut POINT: Option<Arc<Point>> = None;

        unsafe {// Rust中使用可变静态变量都是unsafe的
            POINT.get_or_insert_with(|| {
                // 初始化单例对象的代码
                Arc::new(Point { x: 0, y: 0 })
            }).clone()
        }
    }

    // 返回&'static Point
    pub fn get_instance2<'staic>() -> &'staic Point {
        static mut POINT: Option<Arc<Point>> = None;

        unsafe {// Rust中使用可变静态变量都是unsafe的
            POINT.get_or_insert_with(|| {
                // 初始化单例对象的代码
                Arc::new(Point { x: 0, y: 0 })
            });
            POINT.as_ref().unwrap()
        }
    }

    let xx = get_instance1();
    println!("{:?}", xx);
    let x = get_instance2();
    println!("{:?}", x);
}

#[test]
fn staticLocal() {
    //静态局部变量
    fn haha() {
        static mut x: i32 = 0;
        unsafe {
            if x == 0 {
                println!("haha");
                x = 3;
            }
        }
        println!("--");
    }
    haha();
    haha();
}

#[test]
fn useLazyStatic() {
    use std::collections::HashMap;
    lazy_static! {
       static ref HASHMAP: HashMap<u32, &'static str> = {
            let mut m = HashMap::new();
            m.insert(0, "foo");
            m.insert(1, "bar");
            m.insert(2, "baz");
            m
        };
    }
    // First access to `HASHMAP` initializes it
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    // Any further access to `HASHMAP` just returns the computed value
    println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());
}