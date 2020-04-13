const n: usize = 8;
static mut total: i32 = 0;

fn legal(x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x < n as i32 && y < n as i32
}

fn canput(a: &mut [[i32; n]; n], x: usize, y: usize) -> bool {
    for (dx, dy) in [(0, 1), (1, 0), (1, 1), (1i32, -1i32)].iter() {
        for direction in [1, -1].iter() {
            let mut xx = x as i32;
            let mut yy = y as i32;
            loop {
                xx += *dx * *direction;
                yy += *dy * *direction;
                if legal(xx, yy) {
                    if a[xx as usize][yy as usize] != 0 {
                        return false;
                    }
                } else {
                    break;
                }
            }
        }
    }
    true
}

fn show(a: &mut [[i32; n]; n]) {
    for i in 0..n {
        for j in 0..n {
            print!("{} ", a[i][j])
        }
        println!()
    }
    println!("===============")
}

fn go(a: &mut [[i32; n]; n], x: usize, y: usize, queenCount: i32) {
    if x == n {
        if queenCount == n as i32 {
            show(a);
            unsafe {
                total += 1;
            }
        }
        return;
    }
    if y == n {
        go(a, x + 1, 0, queenCount);
        return;
    }
    if canput(a, x, y) {
        a[x][y] = 1;
        go(a, x, y + 1, queenCount + 1);
        a[x][y] = 0;
    }
    go(a, x, y + 1, queenCount);
}

fn main() {
    let mut a = [[0; n]; n];
    //写一个八皇后问题
    unsafe { total = 0; }
    go(&mut a, 0, 0, 0);
    println!("总共有{}种放法", unsafe { total });
}