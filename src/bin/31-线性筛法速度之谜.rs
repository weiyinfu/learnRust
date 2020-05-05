#![allow(non_snake_case)]

use chrono::Local;

/**
百思不得解：为啥线性筛比普通筛慢很多？
*/
pub fn linearPrime(n: usize) -> Vec<bool> {
    let mut a = Vec::<bool>::new();
    let mut primes: Vec<usize> = Vec::with_capacity((n as f64).sqrt() as usize);
    a.resize(n, true);
    for i in 2..n {
        if a[i] {
            primes.push(i);
        }
        let mut j = 0;
        let l = primes.len();
        while j < l {
            let which = primes[j] * i;
            if which >= n {
                break;
            }
            a[which] = false;
            if i % primes[j] == 0 {
                break;
            }
            j += 1;
        }
    }
    return a;
}

pub fn bruteforce(n: usize) -> Vec<bool> {
    let mut b = Vec::<bool>::new();
    b.resize(n, true);
    for i in 2..n {
        if b[i] {
            let mut j = i + i;
            while j < n {
                b[j] = false;
                j += i;
            }
        }
    }
    return b;
}

fn getTime() -> i64 {
    Local::now().timestamp()
}

fn main() {
    let n = 5000_0000;
    let begTime = getTime();
    let _a = linearPrime(n); //37s
    println!("线性筛法用时：{}", getTime() - begTime);
    let begTime = getTime();
    let _b = bruteforce(n); //34s
    println!("暴力法用时：{}", getTime() - begTime);
}
