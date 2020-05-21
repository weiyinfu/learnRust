#[cfg(test)]
mod ayIterate {
    #[test]
    fn useIterator() {
        //for循环的应用：迭代一个迭代器
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
    fn enumerate() {
        for (ind, value) in "one two three".split_whitespace().enumerate() {
            println!("index={},value={}", ind, value);
        }
    }
}
