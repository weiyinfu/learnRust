#[cfg(test)]
#[allow(unused_variables)]
mod testString {
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
        let y = x.chars();//y是一个iterator
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
            if op.is_none() { break; }
            let now = op.unwrap();
            println!("{}", now);
        }
    }

    #[test]
    fn split2vector() {
        //iterator如何映射为vector
        let mut weight = "07 09 10 05 08 04 02 01 06 03 07 09 10 05 08 04 02".split_whitespace().map(|x| {
            x.parse::<i32>().unwrap()
        });
        let mut a: Vec<i32> = Vec::new();
        loop {
            let now = weight.next();
            if now.is_none() { break; }
            let num = now.unwrap();
            a.push(num);
        }
        println!("{:?}", a);
    }
}