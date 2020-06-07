#[cfg(test)]
#[allow(non_camel_case_types)]
mod ADDataTypes {
    use std::mem::{size_of, size_of_val};

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
    fn useChar() {
        let x = 'a';
        println!("{}", x as i32);
        let y = x as i32;
        let b = (y + 1) as u8 as char;
        println!("{}", b)
    }

    #[test]
    fn useListerals() {
        //u8类型的字节字面量,四种进制：2,8,16,10，字符
        let x = b'A';
        println!("{}", x);
        let x = 0b0111;
        println!("{:b}", x);
        let x = 0o23;
        println!("{:o}", x);
        let x = 0x23;
        println!("{:x}", x);
        //使用更美观的字面量
        let x = 0x2ab_c24;
        println!("{:x}", x); //十六进制小写
        println!("{:X}", x); //十六进制大写
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
        //复合数据类型的大小
        println!("sizeof array={}", size_of_val(&[1, 2, 3u8]));
        println!(
            "sizeof tuple={}=sizeof({})+sizeof({})",
            size_of_val(&(1u128, '我')),
            1u128,
            '我'
        );
    }

    #[test]
    #[allow(unused_variables)]
    fn useArray() {
        //数组和切片
        let a = [0; 5];
        let b = [0, 1, 2, 3];
        let c = &a[2..4]; //创建数组a的切片
        println!("a.len()={} c.len()={}", a.len(), c.len());
        let mut a = [[0; 3]; 3]; //创建二维数组
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
        //使用结构体，结构体有三种:元组结构体，C结构体，空结构体（只用来表示一种类型）
        #[derive(Debug)]
        //元组结构体
        struct TupleStruct(String, i32);
        #[derive(Debug)]
        //C结构体
        struct CStruct {
            name: String,
            age: i32,
        }
        #[derive(Debug)]
        //空结构体
        struct EmptyStruct;
        let x = TupleStruct("魏印福".to_string(), 32);
        let y = CStruct {
            name: "魏印福".to_string(),
            age: 17,
        };
        println!(
            "TupleStruct={:?}\nCStruct={:?}\nEmptyStruct={:?}",
            x, y, EmptyStruct
        )
    }

    #[test]
    fn deconstructStructure() {
        struct Foo {
            x: (u32, u32),
            y: u32,
        }

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

    #[test]
    #[allow(unused_variables)]
    fn multilineString() {
        let x = "天下大势，为我所控
哈哈哈
   真好
Rust的多行字符串最为自然，比JavaScript的``，pyhon的\"\"\"都要自然得多
        ";
        println!("{}", x);
        //单字节字符串
        let y = b"hello world";
        let x = r#"不需要转义个字符串\,%"#;
        let x = "需要转义的字符串\\";
    }

    #[test]
    fn tupleEqual() {
        //元组比较运算
        let x = (1, 2);
        let y = (2, 3);
        println!("{} {}", x < y, x == y);
    }
}

#[test]
#[allow(unused_variables)]
fn demo() {
    // boolean type
    let t = true;
    let f: bool = false;

    // char type
    let c = 'c';

    // numeric types
    let x = 42;
    let y: u32 = 123_456;
    let z: f64 = 1.23e+2;
    let zero = z.min(123.4);
    let bin = 0b1111_0000;
    let oct = 0o7320_1546;
    let hex = 0xf23a_b049i64;

    // string types
    let str = "Hello, world!";
    let string = str.to_string();

    // arrays and slices
    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4];
    let ten_zeros: [i64; 10] = [0; 10];

    // tuples
    let tuple: (i32, &str) = (50, "hello");
    let (fifty, _) = tuple;
    let hello = tuple.1;

    // raw pointers
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };

    // functions
    fn foo(x: i32) -> i32 {
        x
    }
    let bar: fn(i32) -> i32 = foo;
}
