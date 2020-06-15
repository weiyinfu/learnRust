struct Queen {
    a: Vec<Vec<i32>>,
    total: i32,
    n: i32,
    directions: [(i32, i32); 4],
}

impl Queen {
    fn new(n: i32) -> Queen {
        let a = vec![vec![0; n as usize]; n as usize];
        let directions = [(0, 1), (1, 0), (1, 1), (1i32, -1i32)];
        let q = Queen {
            a,
            total: 0,
            n,
            directions,
        };
        return q;
    }
    fn show(&self) {
        for i in 0..self.n as usize {
            for j in 0..self.n as usize {
                print!("{} ", self.a[i][j])
            }
            println!()
        }
        println!("===============")
    }

    fn legal(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.n as i32 && y < self.n as i32
    }

    fn canput(&self, x: usize, y: usize) -> bool {
        for (dx, dy) in self.directions.iter() {
            for direction in [1, -1].iter() {
                let mut xx = x as i32;
                let mut yy = y as i32;
                loop {
                    xx += *dx * *direction;
                    yy += *dy * *direction;
                    if self.legal(xx, yy) {
                        if self.a[xx as usize][yy as usize] != 0 {
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

    fn go(&mut self, x: usize, y: usize, queenCount: i32) {
        if x == self.n as usize {
            if queenCount == self.n {
                self.show();
                self.total += 1;
            }
            return;
        }
        if y == self.n as usize {
            self.go(x + 1, 0, queenCount);
            return;
        }
        if self.canput(x, y) {
            self.a[x][y] = 1;
            self.go(x, y + 1, queenCount + 1);
            self.a[x][y] = 0;
        }
        self.go(x, y + 1, queenCount);
    }
    fn run(&mut self) {
        self.go(0, 0, 0);
    }
}

fn main() {
    let mut q = Queen::new(8);
    q.run();
    println!("{}", q.total);
}
