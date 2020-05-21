use std::io::{stdout, Write};

pub fn hello() {
    println!("hello world");
    stdout().write("hello world".as_bytes()).unwrap();
}

#[cfg(test)]
mod AB {
    use std::fmt;
    use std::fmt::{Display, Formatter};

    #[test]
    fn BasicPrint() {
        // 通常情况下，`{}` 会被任意变量内容所替换。
        // 变量内容会转化成字符串。
        println!("{} days", 31);

        // 不加后缀的话，31 就自动成为 i32 类型。
        // 你可以添加后缀来改变 31 的类型。

        // 用变量替换字符串有多种写法。
        // 比如可以使用位置参数。
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

        // 可以使用命名参数。
        println!(
            "{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over"
        );
    }

    #[test]
    fn printNumbers() {
        // 可以在 `:` 后面指定特殊的格式。
        println!("{} of {:b} people know binary, the other half don't", 1, 2);
        println!("binary ：{:b}\noctal:{:o}\n hex:{:x} ", 123, 123, 123);

        // 你可以按指定宽度来右对齐文本。
        // 下面语句输出 "     1"，5 个空格后面连着 1。
        println!("{number:>width$}", number = 1, width = 6);

        // 你可以在数字左边补 0。下面语句输出 "000001"。
        println!("{number:>0width$}", number = 1, width = 6);
        println!("{:03}", 2);
    }

    #[test]
    fn otherFormat() {
        /*
        format用于格式化字符串
        eprint用于把格式化字符串打印到错误输出流
        */
        println!("{}", format!("I 'am {}", "魏印福"));
        eprintln!("打印到error stream");
    }

    #[test]
    //打印debug版的结构体
    fn printStructureDebug() {
        #[derive(Debug)]
        struct Node {
            name: String,
            age: u32,
        }
        let x = Node {
            name: "weiyinfu".to_string(),
            age: 17,
        };
        println!("{:?}", x);
    }

    #[test]
    fn printStructureDebug2() {
        struct Node {
            name: String,
            age: u32,
        }
        impl fmt::Debug for Node {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "name:{},age={}", self.name, self.age)
            }
        }
        let x = Node {
            name: "weiyinfu".to_string(),
            age: 17,
        };
        println!("{:?}", x);
    }

    #[test]
    fn printStructure() {
        struct Node {
            name: String,
            age: u32,
        }
        impl Display for Node {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "name={},age={}", self.name, self.age)
            }
        }
    }

    #[test]
    fn printList() {
        struct List(Vec<i32>);
        impl fmt::Display for List {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "[");
                let mut isFirst = true;
                for i in 0..self.0.len() {
                    if isFirst {
                        isFirst = false
                    } else {
                        write!(f, ",");
                    }
                    write!(f, "{}", self.0[i]);
                }
                write!(f, "]")
            }
        }
        let x = List(vec![1, 2, 3]);
        println!("{}", x);
    }
}
