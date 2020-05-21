/*
Rust中的Enum非常重要，需要特别说明一下。
*/
#[cfg(test)]
#[allow(unused_variables)]
mod AEEnum {
    use std::mem::size_of_val;

    #[test]
    fn useUse() {
        //使用use语句，省掉作用域
        enum Days {
            monday,
            tuesday,
        }
        use Days::{monday, tuesday};
        println!(
            "monday.size={},tuesday.size={}",
            size_of_val(&monday),
            size_of_val(&tuesday)
        )
    }

    #[test]
    fn CStyleEnum() {
        //C语言风格的枚举
        // 该属性用于隐藏对未使用代码的警告。
        #![allow(dead_code)]

        // 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
        enum Number {
            Zero,
            One,
            Two,
        }

        // 拥有显式辨别值（explicit discriminator）的 enum
        enum Color {
            Red = 0xff0000,
            Green = 0x00ff00,
            Blue = 0x0000ff,
        }

        // `enum` 可以转成整形。
        println!("zero is {}", Number::Zero as i32);
        println!("one is {}", Number::One as i32);

        println!("roses are #{:06x}", Color::Red as i32);
        println!("violets are #{:06x}", Color::Blue as i32);
    }

    #[test]
    #[allow(unused_variables)]
    fn enumStruct() {
        /**
        enum的取值可以是结构体
        */
        enum Day {
            xNode(i32),
            User { name: String, age: i32 },
        }
        let x: Day = Day::User {
            name: "weiyinfu".to_string(),
            age: 12,
        };
    }

    #[test]
    fn enumString() {
        //enum的值也可以是字符串
        enum Planet {
            earth(String),
            moon(String),
        }
        let x = Planet::earth("earth".to_string());
        let y = Planet::moon("moon".to_string());
        if let Planet::earth(value) = x {
            println!("发现一个行星{}", value);
        }
    }

    #[test]
    fn manyKind() {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        impl Message {
            fn print(&self) {
                match &self {
                    Message::Quit => {
                        println!("quit");
                    }
                    Message::Move { .. } => {
                        println!("move");
                    }
                    Message::Write(_) => {
                        println!("write");
                    }
                    Message::ChangeColor(_, _, _) => {
                        println!("change color");
                    }
                }
            }
        }
        let x = Message::Quit;
        x.print();
    }
}
