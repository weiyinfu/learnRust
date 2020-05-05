/*
Rust中的切片非常特殊，它不是一份新的独立的数据，而是很高效的数据
切片不拥有所有权。
*/
#[cfg(test)]
mod biSlice {
    use std::string::String;

    #[test]
    fn one() {
        let str = String::from("one two three");
        let x = &str[1..7];
        let y = &x[2..4];//切片的切片
        println!("x={}$", x);
        println!("y={}$", y);
        println!("z={}$", &str[1..7]);
        //使用等号表示闭区间
        println!("z={}$", &str[1..=7]);
    }

    #[test]
    #[allow(unused_assignments,unused_variables)]
    fn sliceWrite() {
        //更改切片之后原数组的值也会发生变化
        let  x = String::from("one two three");
        let mut y = &x[1..4];
        y = "ag";//对切片执行写操作没什么用处
        println!("{}", x);
    }
}