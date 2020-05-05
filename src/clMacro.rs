#[allow(unused_macros)]
macro_rules! newVec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            println!("begin");
            $(
                println!("pushing {}",$x);
                temp_vec.push($x);
            )*
            println!("end");
            temp_vec
        }
    };
}
#[test]
fn newVec() {
    let x = newVec![1, 2, 3, 4];
    println!("{:?}", x);
    let x = newVec!(1, 2, 3, 4);
    println!("{:?}", x);
}
