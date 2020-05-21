#[test]
fn useAssert() {
    let x = 3;
    assert!(x == 3, "something wrong");
    assert_eq!(x, 3, "something wrong");
    assert_ne!(x, 4, "something wrong");
}

#[test]
#[should_panic]
fn useAssert2() {
    let x = 3;
    assert!(x == 3, "something wrong");
    assert_eq!(x, 3, "something wrong");
    assert_ne!(x, 4, "something wrong");
}

#[test]
#[should_panic(expected = "somethign wrong")]
fn useAssert3() {
    let x = 3;
    assert!(x == 3, "something wrong");
}

#[test]
#[ignore]
fn haha() {
    panic!("dont run me");
}

//尝试测试私有函数
fn b() {}

#[cfg(test)]
mod haha {
    use crate::cnTest::b;

    #[test]
    fn testPrivate() {
        b();
    }
}
