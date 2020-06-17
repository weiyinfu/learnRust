#[test]
fn param() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    fn sub(x: i32, y: i32) -> i32 {
        x - y
    }
    //下面的句子是错误的
    // for (f, op) in [(add, "+".to_string()), (sub, "-".to_string())].iter() {
    //     let (x, y) = (3, 4);
    //     println!("{}{}{}={}", x = x, op = *op, y = y, res = f(x, y));
    // }
    for f in [add, sub].iter() {
        println!("{x} {y} {res}", x = 3, y = 4, res = f(3, 4));
    }
}

#[test]
#[allow(unreachable_code, non_snake_case, unused_variables)]
fn diverge() {
    //感叹号的作用是一开始没想好返回值类型的时候，就用感叹号替代
    fn haha() -> ! {
        unreachable!();
    }
    let x: i32 = haha();
    let y: i64 = haha();
}

#[test]
fn deconstruct_in_arguments() {
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
}

#[test]
fn emptyTuple() {
    fn some_fn() {
        ()
    }

    let a: () = some_fn();
    println!("This function returns and you can see this line.")
}

#[test]
fn highOrderFunction() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn apply<F>(f: F, y: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(y) * y
    }

    fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |y| x + y)
    }

    let transform: fn(i32) -> i32 = add_one;
    let f0 = add_one(2i32) * 2;
    let f1 = apply(add_one, 2);
    let f2 = apply(transform, 2);
    println!("{}, {}, {}", f0, f1, f2);

    let closure = |x: i32| x + 1;
    let c0 = closure(2i32) * 2;
    let c1 = apply(closure, 2);
    let c2 = apply(|x| x + 1, 2);
    println!("{}, {}, {}", c0, c1, c2);

    let box_fn = factory(1i32);
    let b0 = box_fn(2i32) * 2;
    let b1 = (*box_fn)(2i32) * 2;
    let b2 = (&box_fn)(2i32) * 2;
    println!("{}, {}, {}", b0, b1, b2);

    let add_num = &(*box_fn);
    let translate: &dyn Fn(i32) -> i32 = add_num;
    let z0 = add_num(2i32) * 2;
    let z1 = apply(add_num, 2);
    let z2 = apply(translate, 2);
    println!("{}, {}, {}", z0, z1, z2);
}

#[test]
fn test_mut_param() {
    //下面函数参数中的mut是复制了一份数据
    fn haha(mut x: i32) {
        x = 4;
    }
    fn f(mut a: Vec<i32>) {
        a[0] = 10;
    }

    let x = 3;
    haha(x);
    println!("{}", x);
    let a = vec![1, 2, 3];
    //在调用f的时候发生move，a对象在下文中禁止再用
    f(a);
}
