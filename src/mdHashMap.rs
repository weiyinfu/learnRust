use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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
    fn useZip() {
        use std::collections::HashMap;

        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
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

#[test]
fn test1() {
    //查询的时候需要传引用
    let mut ma = HashMap::new();
    let x = 3;
    ma.insert(x, 1); //Rust会根据insert函数来推断HashMap的类型
    let mut p = 2;
    p += 1;
    //在执行get操作的时候，必须使用引用，避免不必要的数据move
    println!("{}", ma.get(&p).unwrap());
}

#[test]
fn get_mut_must_exists() {
    //会报错
    let mut ma: HashMap<&str, i32> = HashMap::new();
    let res = ma.get_mut("one");
    let mut temp = 0;
    let x = res.unwrap_or(&mut temp);
    *x = 5;
    println!("{}", ma.get("one").unwrap());
}

#[test]
fn selfKey() {
    let mut ma = HashMap::new();
    #[derive(Hash, Eq, PartialEq)]
    struct Node;
    let x = Node {};
    ma.insert(x, 0);
}

#[test]
fn update() {
    //根据旧值更新值
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
