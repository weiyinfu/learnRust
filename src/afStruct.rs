#[cfg(test)]
mod afStruct {
    #[test]
    fn structMethods() {
        //为struct添加方法
        struct Triangle {
            a: f64,
            b: f64,
            c: f64,
        }
        impl Triangle {
            fn area(&self) -> f64 {
                let p = (self.a + self.b + self.c) / 2.0;
                return (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt();
            }
        }
        let x = Triangle { a: 2f64, b: 2f64, c: 3f64 };
        println!("{}", x.area());
    }
}