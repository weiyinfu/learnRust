#[test]
fn printArgs() {
    println!("{:?}", std::env::args());
    println!("{:?}", std::env::current_dir());
    println!("{:?}", std::env::args_os());
    println!("{:?}", std::env::current_exe());
    println!("{:?}", std::env::temp_dir());
    println!("{:?}", std::env::var_os("PATH").unwrap());
    for i in std::env::vars_os() {
        println!("{} = {}", i.0.to_str().unwrap(), i.1.to_str().unwrap())
    }
}