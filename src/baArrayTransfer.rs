/**
让数组中的元素的值增加一
*/
#[cfg(test)]
mod baArrayTransfer {
    #[test]
    fn addOne() {
        fn add(mut a: [i32; 3]) {
            for i in 0..a.len() {
                a[i] += 1;
                println!("haha{}", a[i]);
            }
        }
        let a = [1, 2, 3];
        add(a);//此处传值
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