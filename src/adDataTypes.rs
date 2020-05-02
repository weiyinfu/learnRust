#[cfg(test)]
#[warn(non_camel_case_types)]
mod ADDataTypes {
    use std::mem::size_of_val;

    #[test]
    fn basicDataType() {
        //基本数据类型
        println!("i8size={}", size_of_val(&12i8));
        println!("i16size={}", size_of_val(&12i16));
        println!("i32size={}", size_of_val(&12i32));
        println!("i64size={}", size_of_val(&12i64));
        println!("i128size={}", size_of_val(&12i128));
        println!("isize={}", size_of_val(&12isize));
        println!("==========无符号系列");
        println!("u8size={}", size_of_val(&12u8));
        println!("u16size={}", size_of_val(&12u16));
        println!("u32size={}", size_of_val(&12u32));
        println!("u64size={}", size_of_val(&12u64));
        println!("u128size={}", size_of_val(&12u128));
        println!("usize={}", size_of_val(&12usize));
        println!("==========浮点类型");
        println!("f32={}", size_of_val(&123f32));
        println!("f64={}", size_of_val(&123f64));
        println!("=======char系列，一个char是一个int");
        println!("char={}", size_of_val(&'魏'));
        println!("======bool值");
        println!("bool={}", size_of_val(&true));
    }

    #[test]
    fn useListerals() {
        //u8类型的字节字面量
        let x = b'A';
        println!("{}", x);
        let x = 0x23;
        println!("{:x}", x);
        let x = 0o23;
        println!("{:o}", x);
        //使用更美观的字面量
        let x = 0x232_342_34;
        println!("{:x}", x);
    }

    #[test]
    fn useTuple() {
        let mut x = (1, 2, 3);
        println!("{},{},{}", x.0, x.1, x.2);
        //元组的解包
        let (a, b, c) = x;
        println!("{},{},{}", a, b, c);
        //元组是可以改变值的(前提是元组必须是可变元组)，这一点与Python不同
        x.0 = 100;
        println!("{:?}", x)
    }

    #[test]
    fn printTuple() {
        for i in [(1, 2), (3, 4)].iter() {
            println!("{:?}", i)
        }
    }

    #[test]
    fn compoundDataType() {
        //复合数据类型
        println!("sizeof array={}", size_of_val(&[1, 2, 3u8]));
        println!("sizeof tuple={}=sizeof({})+sizeof({})", size_of_val(&(1u128, '我')), 1u128, '我');
    }

    #[test]
    fn useArray() {
        //数组和切片
        let mut a = [0; 5];
        let b = [0, 1, 2, 3];
        let mut c = &a[2..4];//创建数组a的切片
        println!("a.len()={} c.len()={}", a.len(), c.len());
        let mut a = [[0; 3]; 3];//创建二维数组
        for i in 0..a.len() {
            for j in 0..a[i].len() {
                a[i][j] = i * a[0].len() + j;
            }
        }
        fn show(ar: [[usize; 3]; 3]) {
            for i in 0..ar.len() {
                for j in 0..ar.len() {
                    print!("{} ", ar[i][j]);
                }
                println!();
            }
        }
        show(a);
    }


    #[test]
    fn useStruct() {
        //使用结构体，结构体有三种，元组结构体，C结构体，空结构体（只用来表示一种类型）
        #[derive(Debug)]
        struct TupleStruct(String, i32);
        #[derive(Debug)]
        struct CStruct {
            name: String,
            age: i32,
        }
        #[derive(Debug)]
        struct EmptyStruct;
        let x = TupleStruct("魏印福".to_string(), 32);
        let y = CStruct { name: "魏印福".to_string(), age: 17 };
        println!("TupleStruct={:?}\nCStruct={:?}\nEmptyStruct={:?}", x, y, EmptyStruct)
    }

    #[test]
    fn deconstructStructure() {
        struct Foo { x: (u32, u32), y: u32 }

        // 解构结构体的成员
        let foo = Foo { x: (1, 2), y: 3 };
        let Foo { x: (a, b), y } = foo;

        println!("a = {}, b = {},  y = {} ", a, b, y);

        // 可以解构结构体并重命名变量，成员顺序并不重要

        let Foo { y: i, x: j } = foo;
        println!("i = {:?}, j = {:?}", i, j);

        // 也可以忽略某些变量
        let Foo { y, .. } = foo;
        println!("y = {}", y);

        // 这将得到一个错误：模式中没有提及 `x` 字段
        // let Foo { y } = foo;
    }

    #[test]
    fn useEnum() {
        enum WebEvent {
            // 一个 `enum` 可以是单元结构体（称为 `unit-like` 或 `unit`），
            PageLoad,
            PageUnload,
            // 或者一个元组结构体，
            KeyPress(char),
            Paste(String),
            // 或者一个普通的结构体。
            Click { x: i64, y: i64 },
        }
        // 此函数将一个 `WebEvent` enum 作为参数，无返回值。
        fn inspect(event: WebEvent) {
            match event {
                WebEvent::PageLoad => println!("page loaded"),
                WebEvent::PageUnload => println!("page unloaded"),
                // 从 `enum` 里解构出 `c`。
                WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
                WebEvent::Paste(s) => println!("pasted \"{}\".", s),
                // 把 `Click` 解构给 `x` and `y`。
                WebEvent::Click { x, y } => {
                    println!("clicked at x={}, y={}.", x, y);
                }
            }
        }
        let pressed = WebEvent::KeyPress('x');
        // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
        let pasted = WebEvent::Paste("my text".to_owned());
        let click = WebEvent::Click { x: 20, y: 80 };
        let load = WebEvent::PageLoad;
        let unload = WebEvent::PageUnload;

        inspect(pressed);
        inspect(pasted);
        inspect(click);
        inspect(load);
        inspect(unload);
    }


    #[test]
    fn useType() {
        //使用类型别名
        type haha = i32;
        let x: haha = 3;
        println!("{}", x);
    }
}
