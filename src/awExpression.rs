#[cfg(test)]
mod AWExpression {
    #[test]
    fn first() {
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
    fn second() {
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
    fn multiLoop() {
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
        //前n项和
        let mut s = 0;
        for i in 0..10 {
            s += i;
        }
        println!("{}", s);
    }

    #[test]
    fn useIterator() {
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
    }

    #[test]
    fn useIntoIter() {
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
        //下面语句报错，因为names已经变成空壳了，names已经被借出去了，循环结束之后就被释放了
        // println!("{}",names.len());
    }

    #[test]
    fn useIterMut() {
        let mut names = vec!["Bob", "Frank", "Ferris"];
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }
        println!("names: {:?}", names);
    }


    #[test]
    fn useMatch() {
        let number = 13;
        // 试一试 ^ 将不同的值赋给 `number`

        println!("Tell me about {}", number);
        match number {
            // 匹配单个值
            1 => println!("One!"),
            // 匹配多个值
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
            // 匹配一个闭区间范围
            13..=19 => println!("A teen"),
            // 处理其他情况
            _ => println!("Ain't special"),
        }

        let boolean = true;
        // match 也是一个表达式
        let binary = match boolean {
            // match 分支必须覆盖所有可能的值
            false => 0,
            true => 1,
            // 试一试 ^ 将其中一条分支注释掉
        };

        println!("{} -> {}", boolean, binary);
    }

    #[test]
    fn matchDeconstructTuple() {
        let pair = (0, -2);
        // 试一试 ^ 将不同的值赋给 `pair`

        println!("Tell me about {:?}", pair);
        // match 可以解构一个元组
        match pair {
            // 解构出第二个值
            (0, y) => println!("First is `0` and `y` is `{:?}`", y),
            (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
            _ => println!("It doesn't matter what they are"),
            // `_` 表示不将值绑定到变量
        }
    }

    #[test]
    fn matchDeconstructEnum() {
        enum Color {
            // 这三个取值仅由它们的名字（而非类型）来指定。
            Red,
            Blue,
            Green,
            // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
            RGB(u32, u32, u32),
            HSV(u32, u32, u32),
            HSL(u32, u32, u32),
            CMY(u32, u32, u32),
            CMYK(u32, u32, u32, u32),
        }
        let color = Color::RGB(122, 17, 40);
        // 试一试 ^ 将不同的值赋给 `color`

        println!("What color is it?");
        // 可以使用 `match` 来解构 `enum`。
        match color {
            Color::Red => println!("The color is Red!"),
            Color::Blue => println!("The color is Blue!"),
            Color::Green => println!("The color is Green!"),
            Color::RGB(r, g, b) =>
                println!("Red: {}, green: {}, and blue: {}!", r, g, b),
            Color::HSV(h, s, v) =>
                println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
            Color::HSL(h, s, l) =>
                println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
            Color::CMY(c, m, y) =>
                println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
            Color::CMYK(c, m, y, k) =>
                println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                         c, m, y, k),
            // 不需要其它分支，因为所有的情形都已覆盖
        }
    }

    #[test]
    fn matchGuard() {
        //match的守卫
        let pair = (2, -2);
        // 试一试 ^ 将不同的值赋给 `pair`

        println!("Tell me about {:?}", pair);
        match pair {
            (x, y) if x == y => println!("These are twins"),
            // ^ `if` 条件部分是一个守卫
            (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
            (x, _) if x % 2 == 1 => println!("The first one is odd"),
            _ => println!("No correlation..."),
        }
    }


    #[test]
    fn matchBind() {
        //match把数据绑定到特定的变量
        println!("Tell me type of person you are");
        fn age() -> u32 {
            15
        }
        match age() {
            0 => println!("I'm not born yet I guess"),
            // 可以直接 `match` 1 ... 12，但怎么把岁数打印出来呢？
            // 相反，在 1 ... 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
            n @ 1...12 => println!("I'm a child of age {:?}", n),
            n @ 13...19 => println!("I'm a teen of age {:?}", n),
            // 不符合上面的范围。返回结果。
            n => println!("I'm an old person of age {:?}", n),
        }
    }

    #[test]
    fn matchBind2() {
        fn some_number() -> Option<u32> {
            Some(42)
        }
        match some_number() {
            // Got `Some` variant, match if its value, bound to `n`,
            // is equal to 42.
            Some(n @ 42) => println!("The Answer: {}!", n),
            // Match any other number.
            Some(n) => println!("Not interesting... {}", n),
            // Match anything else (`None` variant).
            _ => (),
        }
    }

    #[test]
    fn matchDeconstructPointerAndReference() {
        // 获得一个 `i32` 类型的引用。`&` 表示取引用。
        let reference = &4;

        match reference {
            // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：
            // `&i32`（译注：即 `reference` 的类型）
            //    |
            // `&val`（译注：即用于匹配的模式）
            // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。
            // 译注：因此可用 `val` 表示被 `reference` 引用的值 4。
            &val => println!("Got a value via destructuring: {:?}", val),
        }

        // 如果不想用 `&`，需要在匹配前解引用。
        match *reference {
            val => println!("Got a value via dereferencing: {:?}", val),
        }

        // 如果一开始就不用引用，会怎样？ `reference` 是一个 `&` 类型，因为赋值语句
        // 的右边已经是一个引用。但下面这个不是引用，因为右边不是。
        let _not_a_reference = 3;

        // Rust 对这种情况提供了 `ref`。它更改了赋值行为，从而可以对具体值创建引用。
        // 下面这行将得到一个引用。
        let ref _is_a_reference = 3;

        // 相应地，定义两个非引用的变量，通过 `ref` 和 `ref mut` 仍可取得其引用。
        let value = 5;
        let mut mut_value = 6;

        // 使用 `ref` 关键字来创建引用。
        // 译注：下面的 r 是 `&i32` 类型，它像 `i32` 一样可以直接打印，因此用法上
        // 似乎看不出什么区别。但读者可以把 `println!` 中的 `r` 改成 `*r`，仍然能
        // 正常运行。前面例子中的 `println!` 里就不能是 `*val`，因为不能对整数解
        // 引用。
        match value {
            ref r => println!("Got a reference to a value: {:?}", r),
        }

        // 类似地使用 `ref mut`。
        match mut_value {
            ref mut m => {
                // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
                *m += 10;
                println!("We added 10. `mut_value`: {:?}", m);
            }
        }
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
}