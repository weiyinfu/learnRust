#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Hash)]
struct Node {}

#[test]
fn testOS() {
    // 这个函数仅当目标系统是 Linux 的时候才会编译
    #[cfg(target_os = "linux")]
    fn are_you_on_linux() {
        println!("You are running linux!")
    }

    // 而这个函数仅当目标系统 **不是** Linux 时才会编译
    #[cfg(not(target_os = "linux"))]
    fn are_you_on_linux() {
        println!("You are *not* running linux!")
    }

    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
