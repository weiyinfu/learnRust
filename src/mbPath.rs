#[cfg(test)]
mod testPath {
    use std::path::Path;
    use std::path::PathBuf;

    #[test]
    fn usePath() {
        let filepath = "./Cargo.toml";
        println!("{}", filepath);
        let path = Path::new(filepath);
        println!("{}", path.display());
        println!("is_absolute={}", path.is_absolute());
        println!("文件分隔符{}", std::path::MAIN_SEPARATOR);
        println!("文件是否存在{}", path.exists());
        //如果文件不存在，就会报错
        let abspath = path.canonicalize().unwrap();
        println!("绝对路径:{}", abspath.display());
        println!("是不是文件{}", path.is_file());
        println!(
            "当前路径{}",
            Path::new(".").canonicalize().unwrap().display()
        );
    }

    #[test]
    fn pathJoin() {
        let mut x = PathBuf::from("./");
        x.push("..");
        x.push("..");
        println!("absolute path={}", x.canonicalize().unwrap().display())
    }
}
