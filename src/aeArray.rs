#[test]
fn testArray() {
    let a = [3; 4];
    for i in 0..a.len() {
        println!("a{i}={value}", i = i, value = a[0])
    }
}

#[test]
fn testMultiArray() {
    //测试多维数组
    const x: i32 = 3;
    let mut a = [[3; x as usize]; x as usize];
    a[1][1] = 2;
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            print!("{},", a[i][j]);
        }
        println!()
    }
}
