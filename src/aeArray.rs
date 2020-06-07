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
/**
让数组中的元素的值增加一
*/
#[cfg(test)]
mod baArrayTransfer {
    #[test]
    fn addOne() {
        //这个例子只进行传值，会发生复制，add函数无法对外部的数组执行+1操作
        fn add(mut a: [i32; 3]) {
            for i in 0..a.len() {
                a[i] += 1;
                println!("haha{}", a[i]);
            }
        }
        let a = [1, 2, 3];
        add(a); //此处传值
        for i in 0..a.len() {
            println!("a[{}]={}", i, a[i]);
        }
    }

    #[test]
    fn addOne2() {
        fn add(a: &mut [i32; 3]) {
            for i in 0..a.len() {
                a[i] += 1;
                println!("haha{}", a[i]);
            }
        }
        let mut a = [1, 2, 3];
        //此处传指针
        add(&mut a);
        for i in 0..a.len() {
            println!("a[{}]={}", i, a[i]);
        }
    }
}
