#[cfg(test)]
#[warn(unreachable_code)]
mod AWExpression {
    #[test]
    fn blockExpression() {
        //使用一个语句块作为一个表达式
        let x = 5u32;

        let y = {
            let x_squared = x * x;
            let x_cube = x_squared * x;

            // 将此表达式赋给 `y`
            x_cube + x_squared + x
        };

        let z = {
            // 分号结束了这个表达式，于是将 `()` 赋给 `z`
            2 * x;
        };

        println!("x is {:?}", x);
        println!("y is {:?}", y);
        println!("z is {:?}", z);
    }

    #[test]
    fn ifBlock() {
        //使用if语句块作为表达式
        let x = 3;
        let y = if x < 5 {
            7
        } else {
            8
        };
        println!("{}", y);
    }


    #[test]
    fn useLoop() {
        //使用循环语句块作为表达式
        let mut s = 0;
        let mut i = 0;
        //1+2+...n>100，求最小的n
        let x = loop {
            if s > 100 {
                break i;
            }
            s += i;
            i += 1;
        };
        println!("{}", x);
    }

    #[test]
    #[allow(unreachable_code)]
    #[allow(dead_code)]
    #[allow(unused_labels)]
    fn breakMultiLoop() {
        //Rust的语句标号开头必须是’
        'outer: loop {
            println!("Entered the outer loop");
            'inner: loop {
                println!("Entered the inner loop");
                // 这只是中断内部的循环
                //break;
                // 这会中断外层循环
                break 'outer;
            }

            println!("This point will never be reached");
        }
        println!("Exited the outer loop");
    }


    #[test]
    fn useFor() {
        let mut s = 0;
        let mut x: i32 = 0;
        for i in 0.. {
            s += i;
            if s > 100 {
                x = i;
                break;
            }
        };
        println!("{}", x);
    }

    #[test]
    fn preSum() {
        //for循环的应用：求前n项和
        let mut s = 0;
        for i in 0..10 {
            s += i;
        }
        println!("{}", s);
    }
}