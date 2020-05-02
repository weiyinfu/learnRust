#[cfg(test)]
mod testHashMap {
    use std::collections::HashMap;
    use std::collections::hash_map::RandomState;

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
}