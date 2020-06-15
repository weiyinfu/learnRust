use std::sync::Arc;

/*
闭包实际上是一个结构体

如果有move，则定义的时候就发生所有权转移。
如果没有move，则执行的时候发生所有权转移。
发生move的时候，如果元素在栈上，则复制一份。外部函数依然保有可读权利。
发生move的时候，如果元素在堆上，则转移所有权，外部函数无法再访问元素。
*/
#[test]
fn one() {
    let num = 1;
    let add = |x: i32| x + num;
    println!("{}", add(2));
}

#[test]
fn three() {
    let mut num = 1;
    //add函数会修改闭包的值，所以必须带着mut
    let mut add = |x: i32| num += x;
    add(2);
    println!("{}", num);
}

#[test]
fn threethree() {
    let mut num = vec![1];
    let mut add = |x: i32| num[0] += x;
    add(2);
    println!("{:?}", num);
}

#[test]
fn two() {
    let mut num = 1;
    let mut add = move |x: i32| num += x;
    add(2);
    //输出为1，因为add中的num只是一份复制(因为有move参数)，每当定义一个闭包，都相当于定义结构体
    println!("{}", num);
}

#[test]
fn twotwo() {
    //如果num在栈中，那么move到闭包会使得栈中元素变得只读；如果num在堆中，那么move到闭包会使得失去对元素的一切控制
    let mut num = vec![1];
    let mut add = move |x: i32| num[0] += x;
    add(2);
    //num已经交给add了，当前函数再查看就会报错
    // println!("{:?}", num);
}

#[test]
fn four() {
    //一旦声明为move，那么闭包就固定了，不再发生改变
    let mut a = [1, 2, 3];
    //此处的move是把a复制一份（因为数组a在栈上）
    let check = move || a.iter().sum::<i32>();
    println!("{}", check());
    let mut update = || {
        a[0] += 1;
        println!("在update中{:?}", a);
    };
    update();
    println!("{}", check());
    println!("在外界{:?}", a);
}

#[test]
fn five() {
    //栈中变量的move，把元素复制一份
    let mut a = 1;
    let check = move || a;
    a += 1;
    println!("{}", check()); //输出1
    println!("{}", check()); //输出1
    println!("{:?}", a); //输出2
}

#[test]
#[allow(unused_variables, unused_assignments)]
fn six() {
    let a = 1;
    let mut b = 1;
    let check = || a;
    println!("{}", check()); //输出1
                             //这句话编译报错，因为a的所有权已经交给check这个闭包了
                             // a += 1;
    b += 1;
    println!("{}", check()); //输出1
    println!("{:?}", a); //输出2
}

#[test]
#[allow(unused_mut)]
fn seven() {
    //一旦声明为move，那么闭包就固定了，不再发生改变
    let mut a = [1, 2, 3];
    let mut b = [1, 2, 3];
    let check = || -> i32 { (&a).iter().sum::<i32>() };
    println!("{}", check());
    //a已经交给check b包了，何时释放a，需要由check b包来决定。
    // let mut update = || {
    //     a[0] += 1;
    //     println!("{:?}", a);
    // };
    b[0] = 100; //因为check与b无关，所以b得以幸免
    println!("{}", check());
    println!("{:?}", a);
}

#[test]
fn eight() {
    let mut a = 1;
    let mut add = || a += 1;
    add();
    //下面a+=1和add()只能执行其中一句，因为add是写借用，a.add_assign(&mut self)也是写借用
    a += 1;
    //再次借用就会报错
    // add();
    println!("{}", a);
}

#[test]
#[allow(unused_variables, unused_mut)]
fn eightEight() {
    let mut a = vec![1];
    let mut add = || a[0] += 1;
    //下面两句话顺序反了就会报错
    // a[0] += 1;
    //再次借用就会报错
    add();
    println!("{:?}", a);
}

#[test]
fn nine() {
    let mut a = vec![1];
    let mut add = || a[0] += 1;
    a[0] += 1;
    // add();
    println!("{:?}", a);
}

#[test]
fn doReleaseAfterCapture() {
    use std::mem;

    // b是不可复制类型，因此按值捕获时所有权会转移
    let b = Box::new(12);
    let f = || {
        println!("b = {}", b);
        // drop函数取T类型，因此闭包会按值捕获变量b
        mem::drop(b);
    };
    f();
    //再次调用f会报错，因为b在第一次调用结束之后已经释放了
    // f();
}
/*
三种函数实际上是三种trait
pub trait FnOnce<Args> {
     /// The returned type after the call operator is used.
     #[stable(feature = "fn_once_output", since = "1.12.0")]
     type Output;
 ​
     /// Performs the call operation.
     #[unstable(feature = "fn_traits", issue = "29625")]
     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
 }
 pub trait FnMut<Args>: FnOnce<Args> {
     /// Performs the call operation.
     #[unstable(feature = "fn_traits", issue = "29625")]
     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
 }
 pub trait Fn<Args>: FnMut<Args> {
     /// Performs the call operation.
     #[unstable(feature = "fn_traits", issue = "29625")]
     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
 }
*/
#[allow(dead_code, unused_variables)]
mod ThreeFn {
    fn is_fn<F: Fn()>(f: F) {}

    fn is_fn_mut<F: FnMut()>(f: F) {}

    fn is_fn_once<F: FnOnce()>(f: F) {}

    #[test]
    fn useFnOnce() {
        let s = 0;
        let closure = || {
            s;
        };
        // is_fn(closure);//报错
        is_fn_once(closure);
    }

    #[test]
    fn useFnMut() {
        let mut s = 0;
        let closure = || {
            s = 3;
        };
        // is_fn(closure);//报错
        is_fn_mut(closure);
    }

    #[test]
    fn useFn() {
        let s = String::from("hello");
        let closure = || {
            &s;
        };
        is_fn(closure);
        is_fn_mut(closure);
        is_fn_once(closure);
    }
}

#[cfg(test)]
mod Factorial {
    //使用闭包实现递归调用
    #[test]
    fn useLocalFunction() {
        fn fac(x: i32) -> i32 {
            if x == 0 {
                return 1;
            }
            return x * fac(x - 1);
        }
        println!("{}", fac(3));
    }

    /**
    闭包无法实现递归
    */
    // #[test]
    // fn fac2() {
    //     let fac = |f: Fn, x| -> i32{
    //         x * f(f, x - 1)
    //     };
    //     println!(fac(fac, 3));
    // }
    #[test]
    fn useClosure() {
        /**
        因为闭包函数无法实现递归，所以只能使用结构体实现递归
        */
        struct Fact<'s> {
            f: &'s dyn Fn(&Fact, u32) -> u32,
        }
        let fact = Fact {
            f: &|fact, x| if x == 0 { 1 } else { x * (fact.f)(fact, x - 1) },
        };

        println!("{}", (fact.f)(&fact, 5));
    }

    #[test]
    fn secondClosureFactorial() {
        /**
        因为闭包函数无法实现递归，所以只能使用结构体实现递归
        */
        struct Fact<'s> {
            f: &'s dyn Fn(&Fact, u32, u32),
        }
        let fact = Fact {
            f: &|fact, x, y| {
                if x == 0 {
                    println!("{}", y)
                } else {
                    (fact.f)(fact, x - 1, x * y)
                }
            },
        };

        (fact.f)(&fact, 5, 1);
    }
}

#[test]
fn closureAsParameter() {
    // 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它。
    fn call_me<F: Fn()>(f: F) {
        f()
    }

    // 定义一个满足 `Fn` 约束的封装函数（wrapper function）。
    fn function() {
        println!("I'm a function!");
    }

    // 定义一个满足 `Fn` 约束的闭包。
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}

#[test]
fn returnAClosure() {
    //返回一个闭包函数
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();

        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}

#[test]
fn useClosure() {
    //在实际工作中使用闭包
    let a = vec![1, 2, 3, 4, 5];
    println!("{:?}", a.iter().filter(|x| **x > 3));
    println!("{:?}", a.iter().any(|x| *x > 3));
}

#[test]
fn manyClosure() {
    let mut x = Box::new(3);
    let mut a = || {
        let b = || {
            println!("{}", *x);
        };
        //下面两句调换顺序就会报错
        b();
        *x += 3;
    };
    a();
    println!("{}", *x);
}
