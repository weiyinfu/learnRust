const x: i32 = 3;
static y: i32 = 4;
static mut z: i32 = 5;

fn main() {
    println!("x={},y={}", x, y);
    //static默认为常量
    // y=5;
    unsafe {
        z = 6;
        println!("{}", z);
    }
}