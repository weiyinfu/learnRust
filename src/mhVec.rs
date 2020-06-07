#[test]
#[allow(unused_variables)]
fn createVec() {
    let a = Vec::<i32>::new();
    let a: Vec<i32> = Vec::new();
    //从Ascii码创建u8数组
    let a = Vec::from("1 2 3 4");
    println!("{:?}", a);
    let a = vec![1, 2, 3, 4, 5];
    let a = [3; 4]; //创建数组，用3填充
    let a = vec![3; 4];
    println!("{:?}", a);
}

#[test]
fn visitElement() {
    //访问vec中的元素，两种方式：下表访问和get方法
    let a = vec![1, 2, 3, 4];
    println!("{:?} {} {}", a, a[1], a.get(1).unwrap());
    // println!("{}",a[10]);
    println!("{}", a.get(10).unwrap_or(&-1));
}

#[test]
fn add() {
    //向vec添加元素
    let mut a: Vec<i32> = Vec::new();
    fn show(a: Vec<i32>) {
        println!("{:?}", a);
    }
    //插入的下标必须小于长度
    // a.insert(2, 3);
    a.push(1);
    a.push(2);
    a.insert(1, 3);
    show(a);
}

#[test]
fn iterateByValue() {
    //传值法iterate
    let a = vec![1, 2, 3, 4];
    for i in a {
        println!("{}", i);
    }
    //此处不可打印，因为a已经被move了
    // println!("{:?}",a);
}

#[test]
fn moveIterate() {
    //传引用法iterate
    let a = vec![1, 2, 3, 4];
    for i in &a {
        println!("{}", i)
    }
    println!("{:?}", a);
}

#[test]
fn update() {
    let mut a = vec![1, 2, 3, 4];
    for i in &mut a {
        *i += 3;
    }
    println!("{:?}", a);
}

#[test]
fn dedup() {
    //消重
    let mut a = vec![1, 1, 2, 2, 3];
    a.dedup_by(|x, y| *x == *y);
    println!("{:?}", a);
    let mut a = vec![1, 1, 2, 2, 3];
    a.dedup_by_key(|x| *x);
    println!("{:?}", a);
}

#[test]
fn dedup2() {
    //需要注意，dedup之前一定要排好序
    let mut a = vec![1, 3, 2, 3];
    a.dedup_by_key(|x| *x);
    println!("{:?}", a);
    a.sort();
    a.dedup_by_key(|x| *x);
    println!("{:?}", a);
}

#[test]
fn duckArray() {
    #[derive(Debug)]
    enum Values {
        A(i32),
        B(f64),
        C(String),
    }
    let v = vec![
        Values::A(5),
        Values::B(10.7),
        Values::C(String::from("haha")),
    ];
    for i in v {
        println!("{:?}", i);
    }
}

#[test]
fn iteratorCollect() {
    let data1 = &[3, 1, 4, 1, 5, 9, 2, 6];
    let data2 = &[2, 7, 1, 8, 2, 8, 1, 8];
    let numbers: Vec<i32> = data1
        .iter() // {3, 1, 4...}
        .zip(data2.iter()) // {(3, 2), (1, 7)...}
        .map(|(a, b)| a * b) // {6, 7, ...}
        .filter(|n| *n > 5) // {6, 7....}
        .take(4) // 总共取4个
        .collect();
    println!("{:?}", numbers);
}

#[test]
#[allow(unused_mut)]
fn resize() {
    let mut a = Vec::<i32>::new();
    println!("{:?}", a);
    a.resize(5, 0);
    println!("{:?}", a);
    let mut b = Vec::<i32>::with_capacity(5);
    println!("{:?}", b);
}

#[test]
fn createArray() {
    const n: usize = 10;
    //如果改成let就会报错
    // let n: usize = 10;
    let a = [false; n];
    println!("{:?}", a);
}

#[test]
fn binarySearch() {
    let a = vec![1, 3, 3, 3, 4, 5];
    //error表示没有找到，但是会返回该元素应该插入的位置
    let res = a.binary_search(&2);
    println!("{:?}", res);
    //ok表示找到了，表示该元素的下界
    let res = a.binary_search(&4);
    println!("{:?}", res);

    //ok表示找到了，表示该元素的上确界（闭区间上界）
    let res = a.binary_search(&3);
    println!("{:?}", res);
}
