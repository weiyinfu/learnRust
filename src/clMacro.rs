/**
宏指示符种类：
* block
* expr 用于表达式
* ident 用于变量名或函数名
* item
* pat (模式 pattern)
* path
* stmt (语句 statement)
* tt (标记树 token tree)
* ty (类型 type)


*/
#[test]
fn newVec() {
    //宏的定义，既可以使用中括号，又可以使用小括号
    #[allow(unused_macros)]
    macro_rules! newVec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                println!("begin");
                $(
                    println!("pushing {}",$x);
                    temp_vec.push($x);
                )*
                println!("end");
                temp_vec
            }
        };
    }
    let x = newVec![1, 2, 3, 4];
    println!("{:?}", x);
    let x = newVec!(1, 2, 3, 4);
    println!("{:?}", x);
}

#[test]
fn printSizeOfIntegers() {
    use std::collections::HashMap;
    use std::mem::{size_of, size_of_val};
    //使用宏打印各种整型的字节数

    #[allow(unused_macros)]
    macro_rules! printSize {
        ( $( $x:ident ),* ) => {
            {
                println!("begin");
                $(
                    let x:$x=12;
                    println!("{}={}",stringify!($x),size_of_val(&x));
                )*
                println!("end");
            }
        };
    }
    printSize!(i8, i16, i32, i64, i128, i64, isize);
    printSize!(u8, u16, u32, u64, u128, u64, usize);
}

#[test]
fn createHashMap() {
    use std::collections::HashMap;
    #[allow(unused_macros)]
    macro_rules! map {
        ( $( $x:expr,$y :expr ),* ) => {
            {
                let mut ma = HashMap::new();
                println!("begin");
                $(
                    ma.insert($x,$y);
                    println!("pushing {}",$x);
                )*
                println!("end");
                ma
            }
        };
    }
    let ma = map!(1, "one", 3, "three", 5, "five");
    println!("{:?}", ma);
}

#[test]
fn reload() {
    macro_rules! test {
        // 参数不需要使用逗号隔开。
        // 参数可以任意组合！
        ($left:expr; and $right:expr) => {
            println!(
                "{:?} and {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left && $right
            )
        };
        // ^ 每个分支都必须以分号结束。
        ($left:expr; or $right:expr) => {
            println!(
                "{:?} or {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left || $right
            )
        };
    }

    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}

#[test]
fn sum() {
    macro_rules! SUM {
        ($($x:expr),*) => {
            {
                let mut s=0;
                $(s+=$x;)*
                s
            }
        };
    }
    let s = SUM!(1, 2, 3, 4);
    println!("{}", s);
}

#[test]
fn print() {
    //实现像Python语言那样的print函数
    macro_rules! PRINT {
        ($($x:expr),*) => {
            $(print!("{} ",$x);)*
            println!()
        };
    }
    PRINT!(1, "world", 3, "hello");
}

#[test]
fn vecOprators() {
    //为数组实现+=，×=，-=三种运算符
    use std::ops::{Add, Mul, Sub};

    macro_rules! assert_equal_len {
        // `tt`（token tree，标记树）指示符表示运算符和标记。
        ($a:ident, $b: ident, $func:ident, $op:tt) => {
            assert!(
                $a.len() == $b.len(),
                "{:?}: dimension mismatch: {:?} {:?} {:?}",
                stringify!($func),
                ($a.len(),),
                stringify!($op),
                ($b.len(),)
            );
        };
    }

    macro_rules! op {
        ($func:ident, $bound:ident, $op:tt, $method:ident) => {
            fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
                assert_equal_len!(xs, ys, $func, $op);

                for (x, y) in xs.iter_mut().zip(ys.iter()) {
                    *x = $bound::$method(*x, *y);
                    // *x = x.$method(*y);
                }
            }
        };
    }

    // 实现 `add_assign`、`mul_assign` 和 `sub_assign` 等函数。
    op!(add_assign, Add, +=, add);
    op!(mul_assign, Mul, *=, mul);
    op!(sub_assign, Sub, -=, sub);
    //
    // use std::iter;
    // macro_rules! test {
    //     ($func: ident, $x:expr, $y:expr, $z:expr) => {
    //         #[test]
    //         fn $func() {
    //             for size in 0usize..10 {
    //                 let mut x: Vec<_> = iter::repeat($x).take(size).collect();
    //                 let y: Vec<_> = iter::repeat($y).take(size).collect();
    //                 let z: Vec<_> = iter::repeat($z).take(size).collect();
    //
    //                 super::$func(&mut x, &y);
    //
    //                 assert_eq!(x, z);
    //             }
    //         }
    //     };
    // }
    // // 测试 `add_assign`、`mul_assign` 和 `sub_assign`
    // test!(add_assign, 1u32, 2u32, 3u32);
    // test!(mul_assign, 2u32, 3u32, 6u32);
    // test!(sub_assign, 3u32, 2u32, 1u32);
}
