/*
Rust中的切片非常特殊，它不是一份新的独立的数据，而是很高效的视图
切片不拥有所有权。
*/
#[cfg(test)]
mod biSlice {
    use std::string::String;

    #[test]
    fn one() {
        let str = String::from("one two three");
        let x = &str[1..7];
        let y = &x[2..4]; //切片的切片
        println!("x={}$", x);
        println!("y={}$", y);
        println!("z={}$", &str[1..7]);
        //使用等号表示闭区间
        println!("z={}$", &str[1..=7]);
    }

    #[test]
    #[allow(unused_assignments, unused_variables)]
    fn sliceWrite() {
        //更改切片之后原数组的值也会发生变化
        let x = String::from("one two three");
        let mut y = &x[1..4];
        y = "ag"; //对切片执行写操作没什么用处，并不会改变原来的数组
        println!("{}", x);
    }

    #[test]
    fn useFor() {
        let s = "haha";
        for i in (&s[..3]).chars() {
            print!("{},", i)
        }
        println!();
        for i in (&s[..]).chars() {
            print!("{},", i)
        }
        println!();
        let x = vec![1, 2, 3, 4];
        for i in (&x[..2]).iter() {
            println!("{}", i);
        }
    }
}

#[test]
fn contains() {
    //判断一个元素位于某个区间
    println!("{}", (1..3).contains(&2));
    println!("{}", (1..3).contains(&3));
    println!("{}", (1..=3).contains(&3));
}

#[test]
fn s() {
    //任意数组的slice，是&[T]类型
    fn f(a: &[i32]) -> i32 {
        //&[T]这种方式既可以接受数组又可以接受vec
        a.iter().sum()
    }
    let a = vec![1, 2, 3, 4];
    let b = [1, 2, 3, 4];
    println!("{}", f(&a));
    println!("{}", f(&b));
    let cc = &a[..];
}
