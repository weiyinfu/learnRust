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
#[test]
fn newVec() {
    //宏的定义，既可以使用中括号，又可以使用小括号
    let x = newVec![1, 2, 3, 4];
    println!("{:?}", x);
    let x = newVec!(1, 2, 3, 4);
    println!("{:?}", x);
}

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
#[test]
fn printSizeOfIntegers() {
    printSize!(i8, i16, i32, i64, i128, i64, isize);
    printSize!(u8, u16, u32, u64, u128, u64, usize);
}
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
#[test]
fn createHashMap() {
    use std::collections::HashMap;
    let ma = map!(1, "one", 3, "three", 5, "five");
    println!("{:?}", ma);
}
