#[test]
fn one() {
    struct Example<'a> {
        x: &'a i32,
    }
    let y = &9;
    let b = Example { x: y };
    println!("{}", b.x);
}

#[test]
fn second() {
    struct Example<'a> {
        x: &'a i32,
    }
    impl<'a> Example<'a> {
        fn display(&self)
        {
            print!("Value of x is : {}", self.x);
        }
    }
    let y = &90;
    let b = Example { x: y };
    b.display();
}

#[test]
fn dropTrait() {
    struct Example {
        a: i32,
    }
    impl Drop for Example {
        fn drop(&mut self) {
            println!("Dropping the instance of Example with data : {}", self.a);
        }
    }
    let a1 = Example { a: 10 };
    let b1 = Example { a: 20 };
    println!("Instances of Example type are created");
}

#[test]
fn handDrop() {
    //手动drop，不能调用a.drop()函数，而应该调用mem中的drop函数
    struct Example {
        a: i32,
    }
    impl Drop for Example {
        fn drop(&mut self) {
            println!("Dropping the instance of Example with data : {}", self.a);
        }
    }
    let a1 = Example { a: 10 };
    let b1 = Example { a: 20 };
    //不允许主动drop
    // a1.drop();
    std::mem::drop(a1);
    println!("Instances of Example type are created");
}