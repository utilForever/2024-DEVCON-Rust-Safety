use std::{thread, time::Duration};

fn main() {
    let handle1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread 1: Running - {i}");
            thread::sleep(Duration::from_millis(500));
        }

        42
    });
    let handle2 = thread::spawn(|| {
        println!("Thread 2: Running");
        panic!("Thread 2: Panic!");
    });

    match handle1.join() {
        Ok(result) => println!("Thread 1 is completed: {result}"),
        Err(e) => println!("Panic occurs in Thread 1: {e:?}"),
    }
    match handle2.join() {
        Ok(_) => println!("Thread 2 is completed"),
        Err(e) => println!("Panic occurs in Thread 2: {e:?}"),
    }
}
