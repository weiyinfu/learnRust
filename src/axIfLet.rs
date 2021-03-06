#[cfg(test)]
mod axIfLet {
    #[test]
    fn badStyle() {
        // 将 `optional` 定为 `Option<i32>` 类型
        let optional = Some(7);

        match optional {
            Some(i) => {
                println!("This is a really long string and `{:?}`", i);
                // ^ 行首需要 2 层缩进。这里从 optional 中解构出 `i`。
                // 译注：正确的缩进是好的，但并不是 “不缩进就不能运行” 这个意思。
            }
            _ => {} // ^ 必须有，因为 `match` 需要覆盖全部情况。不觉得这行很多余吗？
        };
    }

    #[test]
    fn goodStyle() {
        // 全部都是 `Option<i32>` 类型
        let number = Some(7);
        let letter: Option<i32> = None;
        let emoticon: Option<i32> = None;

        // `if let` 结构读作：若 `let` 将 `number` 解构成 `Some(i)`，则执行
        // 语句块（`{}`）
        if let Some(i) = number {
            println!("Matched {:?}!", i);
        }

        // 如果要指明失败情形，就使用 else：
        if let Some(i) = letter {
            println!("Matched {:?}!", i);
        } else {
            // 解构失败。切换到失败情形。
            println!("Didn't match a number. Let's go with a letter!");
        };

        // 提供另一种失败情况下的条件。
        let i_like_letters = false;

        if let Some(i) = emoticon {
            println!("Matched {:?}!", i);
        // 解构失败。使用 `else if` 来判断是否满足上面提供的条件。
        } else if i_like_letters {
            println!("Didn't match a number. Let's go with a letter!");
        } else {
            // 条件的值为 false。于是以下是默认的分支：
            println!("I don't like letters. Let's go with an emoticon :)!");
        };
    }

    #[test]
    fn goodStyleForEnum() {
        // 以这个 enum 类型为例
        enum Foo {
            Bar,
            Baz,
            Qux(u32),
        }

        // 创建变量
        let a = Foo::Bar;
        let b = Foo::Baz;
        let c = Foo::Qux(100);

        // 变量 a 匹配到了 Foo::Bar
        if let Foo::Bar = a {
            println!("a is foobar");
        }

        // 变量 b 没有匹配到 Foo::Bar，因此什么也不会打印。
        if let Foo::Bar = b {
            println!("b is foobar");
        }

        // 变量 c 匹配到了 Foo::Qux，它带有一个值，就和上面例子中的 Some() 类似。
        if let Foo::Qux(value) = c {
            println!("c is {}", value);
        }
    }

    #[test]
    fn whileMatchBadStyle() {
        // 将 `optional` 设为 `Option<i32>` 类型
        let mut optional = Some(0);

        // 重复运行这个测试。
        loop {
            match optional {
                // 如果 `optional` 解构成功，就执行下面语句块。
                Some(i) => {
                    if i > 9 {
                        println!("Greater than 9, quit!");
                        optional = None;
                    } else {
                        println!("`i` is `{:?}`. Try again.", i);
                        optional = Some(i + 1);
                    }
                    // ^ 需要三层缩进！
                }
                // 当解构失败时退出循环：
                _ => {
                    break;
                } // ^ 为什么必须写这样的语句呢？肯定有更优雅的处理方式！
            }
        }
    }

    #[test]
    fn useWhileLet() {
        // 将 `optional` 设为 `Option<i32>` 类型
        let mut optional = Some(0);

        // 这读作：当 `let` 将 `optional` 解构成 `Some(i)` 时，就
        // 执行语句块（`{}`）。否则就 `break`。
        while let Some(i) = optional {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
            // ^ 使用的缩进更少，并且不用显式地处理失败情况。
        }
        // ^ `if let` 有可选的 `else`/`else if` 分句，
        // 而 `while let` 没有。
    }
}

#[test]
fn someValue() {
    //someValue
    let x: Option<i32> = Some(3);
    if let Some(3) = x {
        println!("yes");
    }
    if x == Some(3) {
        println!("yes");
    }
}

#[test]
fn ifLetStruct() {
    /**
    解构是Rust的基本特性，被解构的对象不仅仅可以是枚举，也可以是结构体
    */
    struct Node {
        value: i32,
    }
    let x = Node { value: 4 };
    if let Node { value } = x {
        println!("{}", value);
    }
    match x {
        Node { value: 3 } => println!("value is 3"),
        Node { value } => println!("value is {}", value),
        //因为上面已经穷尽了所有情况，所以，即是没有下面这句话也不会报错
        // _ => println!("it is none"),
    }
}

#[test]
fn ifLetTupleStruct() {
    /**
    解构是Rust的基本特性，被解构的对象不仅仅可以是枚举，也可以是结构体
    */
    struct Node(i32);
    let x = Node(4);
    if let Node(value) = x {
        println!("{}", value);
    }
    match x {
        Node(3) => println!("value is 3"),
        Node(value) => println!("value is {}", value),
        //下面这个分支永远不会执行到
        Node(3) => println!("value is 3"),
    }
}
