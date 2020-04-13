#[cfg(test)]
mod AMVariable {
    #[test]
    fn testVariable() {
        //变量先声明
        let x: i32;
        let y = 4;
        if y < 5 {
            x = 7
        } else {
            x = 8;
        }
        println!("{}", x);
    }
}