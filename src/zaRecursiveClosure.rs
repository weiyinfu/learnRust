#[cfg(test)]
mod baga {
    #[test]
    fn haha() {
        println!("haha");
    }

    #[test]
    fn testArray() {
        let a = [3; 4];
        for i in 0..a.len() {
            println!("a{i}={value}", i = i, value = a[0])
        }
    }

    #[test]
    fn testMultiArray() {
        //测试多维数组
        const x: i32 = 3;
        let mut a = [[3; x as usize]; x as usize];
        a[1][1] = 2;
        for i in 0..a.len() {
            for j in 0..a[i].len() {
                print!("{},", a[i][j]);
            }
            println!()
        }
    }

    #[test]
    fn factorial() {
        fn fac(x: i32) -> i32 {
            if x == 0 {
                return 1;
            }
            return x * fac(x - 1);
        }
        println!("{}", fac(3));
    }

    /**
    闭包无法实现递归
    */
    // #[test]
    // fn fac2() {
    //     let fac = |f: Fn, x| -> i32{
    //         x * f(f, x - 1)
    //     };
    //     println!(fac(fac, 3));
    // }
    #[test]
    fn strange() {
        /**
        因为闭包函数无法实现递归，所以只能使用结构体实现递归
        */
        struct Fact<'s> { f: &'s dyn Fn(&Fact, u32) -> u32 }
        let fact = Fact {
            f: &|fact, x| if x == 0 { 1 } else { x * (fact.f)(fact, x - 1) }
        };

        println!("{}", (fact.f)(&fact, 5));
    }

    #[test]
    fn strange2() {
        /**
        因为闭包函数无法实现递归，所以只能使用结构体实现递归
        */
        struct Fact<'s> { f: &'s dyn Fn(&Fact, u32, u32) }
        let fact = Fact {
            f: &|fact, x, y| if x == 0 { println!("{}", y) } else { (fact.f)(fact, x - 1, x * y) }
        };

        (fact.f)(&fact, 5, 1);
    }

    /*
    下面这种方式也是错误的。
    */
    // #[test]
    // fn useVariable() {
    //     let mut fac = |x: i32| { 0 };
    //     fac = |x: i32| { if x == 0 { 1 } else { x * fac(x - 1) } };
    //     println!("{}", fac(5));
    // }

}
