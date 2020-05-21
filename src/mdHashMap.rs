#[cfg(test)]
mod testHashMap {
    use std::collections::hash_map::RandomState;
    use std::collections::HashMap;

    fn getMap() -> HashMap<String, i32, RandomState> {
        let mut x: HashMap<String, i32> = HashMap::new();
        x.insert("one".to_string(), 1);
        x.insert("two".to_string(), 2);
        return x;
    }

    #[test]
    fn one() {
        let x = getMap();
        println!("HashMap中的元素个数：{}", x.len());
        println!("one={}", x.get("one").unwrap());
    }

    #[test]
    fn two() {
        let x = getMap();
        x.keys().for_each(|k| {
            println!("key={}", k);
        })
    }

    #[test]
    fn createHashMapFromVec() {
        use std::collections::HashMap;

        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        println!("{:?}", scores);
    }

    #[test]
    fn onInsert() {
        //插入之后元素就不能再使用了
        let k = "one";
        let mut v = 1;
        //此处是复制
        let mut ma = HashMap::new();
        ma.insert(k, v);
        v = 4;
        println!("{}", ma.get(k).unwrap());
    }

    #[test]
    fn onInsert2() {
        use std::collections::HashMap;

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        //因为field_name和field_name都是堆上的对象，它们已经被移动到了map中
        // println!("{}", field_name);
    }

    #[test]
    fn iterate() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    #[test]
    fn useEntry() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        //entry函数返回一个Enum，它有一个or_insert函数用于插入，entry提供了避免重复插入的一种简单写法
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }
}
