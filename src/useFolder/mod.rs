mod one;

fn haha() {
    println!("haha");
}

pub use one::haha as baga;
