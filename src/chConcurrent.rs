use std::thread;
use std::time::Duration;
use std::sync::mpsc;

#[allow(dead_code)]
fn spawn_function() {
    for i in 0..5 {
        println!("spawned thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn one() {
    thread::spawn(spawn_function);

    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn two() {
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

#[test]
fn communicate() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}


#[test]
fn sendAndSync() {
    use std::sync::{Arc, Mutex};
    use std::sync::mpsc;
    use std::thread;

    let data = Arc::new(Mutex::new(0u32));

    // Creates a shared channel that can be sent along from many threads
    // where tx is the sending half (tx for transmission),
    // and rx is the receiving half (rx for receiving).
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += i;

            tx.send(*data).unwrap();
        });
    }

    for _ in 0..10 {
        println!("{}", rx.recv().unwrap());
    }
}