/**
好难，如何调通这个程序
*/
#[allow(non_upper_case_globals, dead_code, unused_variables, unused_mut)]
fn main() {
    //写一个八皇后问题
    const n: usize = 8;
    let mut a = [[0; n]; n];
    let legal = |x: &i32, y: &i32| *x >= 0 && *y >= 0 && *x < n as i32 && *y < n as i32;
    let canput = |x: &usize, y: &usize| {
        for (dx, dy) in [(0, 1), (1, 0), (1, 1), (-1i32, -1i32)].iter() {
            let xx = *x as i32;
            let yy = *y as i32;
            for direction in [1, -1].iter() {
                loop {
                    let xx = xx + *dx * *direction;
                    let yy = yy + *dy * *direction;
                    if legal(&xx, &yy) {
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
    };
    let show = || {
        for i in 0..n {
            for j in 0..n {
                print!("{} ", a[i][j])
            }
            println!()
        }
        println!("===============")
    };
    type Board = [[usize; n]];
    struct Go<'s> {
        go: &'s dyn Fn(&Go, usize, usize),
    }
    // let go = Go {
    //     go: &|g, x, y| {
    //         if x == n {
    //             show();
    //             return;
    //         }
    //         if y == n {
    //             (g.go)(&g, x + 1, 0);
    //             return;
    //         }
    //         if canput(&x, &y) {
    //             a[x][y] = 1;
    //             (g.go)(g, x, y + 1);
    //             a[x][y] = 0;
    //         }
    //         let gg = g.go;
    //         gg(g, x, y + 1);
    //     },
    // };
    // (go.go)(&go, 0, 0);
}
