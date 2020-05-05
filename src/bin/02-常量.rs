const X: i32 = 3;
static Y: i32 = 4;
static mut Z: i32 = 5;

fn main() {
    println!("x={},y={}", X, Y);
    //static默认为常量
    // y=5;
    unsafe {
        Z = 6;
        println!("{}", Z);
    }
}
