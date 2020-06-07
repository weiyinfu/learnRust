#[cfg(test)]
mod ayIterate {
    #[test]
    fn useIterator() {
        //for循环的应用：迭代一个迭代器，vec的iter函数的参数为&self，以只读的方式迭代
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
        println!("{:?}", names);
    }

    #[test]
    fn useIntoIter() {
        let names = vec!["Bob", "Frank", "Ferris"];
        //vec.into_iter的参数为(self),会发生move
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
        //iter_mut的参数为&mut self,
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }
        println!("names: {:?}", names);
    }

    #[test]
    fn enumerate() {
        for (ind, value) in "one two three".split_whitespace().enumerate() {
            println!("index={},value={}", ind, value);
        }
    }

    #[test]
    fn fibnaci() {
        //使用iterater接口实现斐波那契数列
        struct Fib(i32, i32);
        impl Iterator for Fib {
            type Item = i32;

            fn next(&mut self) -> Option<Self::Item> {
                let ans = self.0 + self.1;
                self.0 = self.1;
                self.1 = ans;
                return Some(ans);
            }
        }
        impl Fib {
            fn new() -> Fib {
                return Fib(1, 1);
            }
        }
        let x = Fib::new();
        for i in x {
            if i > 100 {
                break;
            }
            println!("{}", i);
        }
    }
}
