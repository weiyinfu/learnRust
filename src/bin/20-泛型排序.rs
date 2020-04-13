use std::mem::swap;
use std::fmt::Debug;

const n: usize = 4;

fn sort<T: PartialOrd + Debug + Copy>(a: &mut [T; n]) {
    for i in 0..a.len() {
        for j in i..a.len() {
            if a[i] > a[j] {
                let temp = a[i];
                a[i] = a[j];
                a[j] = temp;
            }
        }
    }
}

fn show<T: Debug>(a: [T; n]) {
    for i in 0..a.len() {
        print!("{:?},", a[i])
    }
    println!()
}

fn main() {
    let mut a = [1, 3, 2, 4];
    sort(&mut a);
    show(a);
    let mut a = [1.0, 4.0, 2.0, 3.0];
    sort(&mut a);
    show(a);
}