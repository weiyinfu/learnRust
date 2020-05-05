#[cfg(test)]
mod acRandom {
    use rand;
    use rand::Rng;

    #[test]
    fn haha() {
        println!("{}", rand::random::<i32>());
        println!("{}", rand::random::<f32>());
        println!("{}", rand::thread_rng().gen_range(0, 100));
    }

    #[test]
    fn thread_rng() {
        let mut x = rand::thread_rng();
        println!("next={}", x.next_f32());
        println!("next={}", x.next_f64());
        println!("next={}", x.next_u32());
    }

    #[test]
    fn useArray() {
        let mut x = rand::thread_rng();
        let mut a = [0; 10];
        //生成10个随机数
        for i in 0..a.len() {
            a[i] = x.gen_range(0, 100);
        }
        println!("shuffle之前：{:?}", a);
        x.shuffle(&mut a);
        println!("shuffle之后：{:?}", a);
        println!("随机选择一个元素：{:?}", x.choose(&a));
        //使用可变随机选择，选择元素，然后把元素改为0，最后肯定是全部元素都变成了0
        for _ in 0..100 {
            let change = x.choose_mut(&mut a);
            *change.unwrap() = 0;
        }
        println!("更改元素之后：{:?}", a);
    }
}