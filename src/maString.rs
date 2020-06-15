#[cfg(test)]
#[allow(unused_variables)]
mod testString {
    use std::ops::Index;

    #[test]
    fn create() {
        let x = String::new();
        let x = String::from("xxxx");
        let x = "hhh".to_string();
    }

    #[test]
    fn len() {
        let x = String::from("ha天下大势为我所控");
        println!("{}", x.len());
        print!("{}", x.chars().count());
    }

    #[test]
    fn update() {
        let mut x = String::new();
        x.push_str("hello");
        x.push('w');
        x.push_str("orld");
        println!("{}", x);
    }

    #[test]
    fn iterate() {
        let mut x = String::from("hello world");
        for i in x.chars() {
            println!("{}", i);
        }
        let y = x.chars(); //y是一个iterator
        x.insert_str(0, "good");
        println!("{}", x)
    }

    #[test]
    fn stringSplit() {
        let x = "one two three four";
        for word in x.split_whitespace().rev() {
            println!("{}", word);
        }
    }

    #[test]
    fn stringSplit2() {
        let x = "one two three four";
        let res = x.split_whitespace();
        print!("count={}", res.count());
        //如果取消注释下面这一行，就会报错
        // print!("count={}", res.count());
    }

    #[test]
    #[allow(unused_mut)]
    fn stringSplit3() {
        let x = "one two three";
        let mut res = x.split_whitespace();
        for word in res.rev() {
            println!("{}", word);
        }
        //因为res是一个迭代器，如果计算了res的count，就没法再调用res的next了
        // println!(res.count());
    }

    #[test]
    fn stringSplit4() {
        let x = "one two three";
        let mut res = x.split_whitespace();
        loop {
            let op = res.next();
            if op.is_none() {
                break;
            }
            let now = op.unwrap();
            println!("{}", now);
        }
    }

    #[test]
    fn split2vector() {
        //iterator如何映射为vector
        let mut weight = "07 09 10 05 08 04 02 01 06 03 07 09 10 05 08 04 02"
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());
        let mut a: Vec<i32> = Vec::new();
        loop {
            let now = weight.next();
            if now.is_none() {
                break;
            }
            let num = now.unwrap();
            a.push(num);
        }
        println!("{:?}", a);
    }

    #[test]
    fn indexOf() {
        //Rust的字符没有indexOf函数
        let s = "weiyinfu";
        println!("{:?}", s.find("ei").unwrap());
    }
}

#[test]
fn rawString() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果你要在原始字符串中写引号，请在两边加一对 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果字符串中需要写 "#，那就在定界符中使用更多的 #。
    // 可使用的 # 的数目没有限制。
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}

#[test]
fn byteString() {
    // 注意这并不是一个 &str
    let bytestring: &[u8; 20] = b"this is a bytestring";

    // 字节串没有实现 Display，所以它们的打印功能有些受限
    println!("A bytestring: {:?}", bytestring);

    // 字节串可以使用单字节的转义字符...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...但不能使用 Unicode 转义字符
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);

    // 原始字节串和原始字符串的写法一样
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    let quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // 字节串可以不使用 UTF-8 编码
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82"; // SHIFT-JIS 编码的 "ようこそ"
}
