#[cfg(test)]
mod testFile {
    use std::fs::File;
    use std::path::Path;
    use std::io::Read;
    use std::collections::HashMap;

    #[test]
    fn testRead() {
        let p = Path::new("./Cargo.toml");
        let mut file = File::open(p).unwrap();
        let mut content = String::new();
        let fileSize = file.read_to_string(&mut content).unwrap();
        println!("文件大小为{}", fileSize);
        println!("文件内容为{}", content);
    }

    #[test]
    fn readAndSplit() {
        /*
        假设有一个文本文件，里面有2n个字符串，第偶数个字符串是key，第奇数个字符串是value
        建立一个HashMap
        */
        let p = Path::new("./Cargo.toml");
        let mut file = File::open(p).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content);
        let mut res = content.split_whitespace();
        let mut book: HashMap<String, String> = HashMap::new();
        loop {
            let keyOpt = res.next();
            if keyOpt.is_none() {
                break;
            }
            let key = keyOpt.unwrap();
            let valueOpt = res.next();
            if valueOpt.is_none() {
                break;
            }
            let value = res.next().unwrap();
            book.insert(key.to_string(), value.to_string());
        }
        println!("{}", book.len());
    }
}