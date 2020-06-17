use core::fmt;
use std::collections::HashMap;

use serde::export::Formatter;

/**
一个3*3的格点键盘，每个点与其周围8个点相连。手势密码不能包含重复数字。问每种长度的手势密码各有多少个？
八联通
不可以跨越，只能连续
不可以重复
123
456
789
*/
fn legal(x: i32, y: i32) -> bool {
    return (0..3).contains(&x) && (0..3).contains(&y);
}

fn go(dp: &mut Vec<Vec<i64>>, need: usize, pos: usize) -> i64 {
    if dp[pos][need] != -1 {
        return dp[pos][need];
    }
    if need & (1 << pos) == 0 {
        //根本不可能到达此pos
        dp[pos][need] = 0;
        return 0;
    }
    let next_need = need ^ (1 << pos); //清空当前位置
    if next_need == 0 {
        //如果需要去的地方都已经去过了
        dp[pos][need] = 1;
        return 1;
    }
    let x = pos / 3;
    let y = pos % 3;
    let mut ans = 0;
    for dx in [1, 0, -1].iter() {
        for dy in [1, 0, -1].iter() {
            let xx = x as i32 + *dx;
            let yy = y as i32 + *dy;
            if legal(xx, yy) {
                let p = (xx * 3 + yy) as usize;
                //确实未曾访问
                ans += go(dp, next_need, p);
            }
        }
    }
    dp[pos][need] = ans;
    ans
}

type Gesture = Vec<usize>;

fn get_patterns(dp: &mut Vec<Vec<Option<Vec<Gesture>>>>, need: usize, pos: usize) -> &[Gesture] {
    if dp[pos][need].is_some() {
        return dp[pos][need].as_deref().unwrap();
    }
    if need & (1 << pos) == 0 {
        //根本不可能到达此pos
        dp[pos][need] = Some(vec![]);
        return dp[pos][need].as_deref().unwrap();
    }
    let next_need = need ^ (1 << pos); //清空当前位置
    if next_need == 0 {
        //如果需要去的地方都已经去过了
        dp[pos][need] = Some(vec![vec![pos]]);
        return dp[pos][need].as_deref().unwrap();
    }
    let x = pos / 3;
    let y = pos % 3;
    let mut ans = vec![];
    for dx in [1, 0, -1].iter() {
        for dy in [1, 0, -1].iter() {
            let xx = x as i32 + *dx;
            let yy = y as i32 + *dy;
            if legal(xx, yy) {
                let p = (xx * 3 + yy) as usize;
                //确实未曾访问
                let sons = get_patterns(dp, next_need, p);
                for i in sons {
                    let mut it = i.clone();
                    it.insert(0, pos);
                    ans.push(it);
                }
            }
        }
    }
    dp[pos][need] = Some(ans);
    dp[pos][need].as_deref().unwrap()
}

fn bit_count(mut x: i32) -> i32 {
    //一个int中数字1的个数
    let mut cnt = 0;
    while x > 0 {
        if x & 1 == 1 {
            cnt += 1;
        }
        x >>= 1;
    }
    cnt
}

fn print_table() {
    let mut dp = vec![vec![-1i64; 1 << 9]; 9];
    let mut ma = HashMap::new();
    for i in 0..dp.len() {
        for j in 0..dp[i].len() {
            let c = go(&mut dp, j, i);
            let bit = bit_count(j as i32);
            let count = c + *ma.get(&bit).unwrap_or(&0);
            ma.insert(bit, count);
        }
    }
    println!("{:?}", ma);
    println!("{}", go(&mut dp, 0b11, 2));
}

fn verbose_show(ma: &HashMap<i32, Vec<Gesture>>) {
    for i in ma.keys() {
        let gestures = ma.get(i).unwrap();
        for g in gestures {
            println!("{:?}", g.iter().map(|x| *x + 1).collect::<Vec<usize>>());
        }
    }
}

fn main() {
    print_table();
    let mut dp = vec![vec!(None; 1 << 9); 9];
    let mut ma: HashMap<i32, Vec<Gesture>> = HashMap::new();
    for i in 0..dp.len() {
        for j in 0..dp[i].len() {
            let mut c = get_patterns(&mut dp, j, i).clone().to_vec();
            let bit = bit_count(j as i32);
            let mut count = ma.get(&bit).unwrap_or(&mut vec![]).clone();
            for g in c {
                count.push(g);
            }
            ma.insert(bit, count.clone());
        }
    }
    for i in ma.keys() {
        println!("i={} count={}", i, ma.get(i).unwrap().len());
    }
    verbose_show(&ma);
}
