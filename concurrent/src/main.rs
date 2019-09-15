use std::thread;
use std::time::Duration;
use std::sync::{Mutex, mpsc, Arc};

fn main() {
    thing();
    mutex_simple();
    mutex_complex()
}

fn mutex_simple() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 2;
    }
    println!("MUTEX IS {:?}", m)
}

fn mutex_complex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn simple(){
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 { 
        println!("hi number {} from main", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

fn thing(){
    
let (tx, rx) = mpsc::channel();

let tx1 = mpsc::Sender::clone(&tx);
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}
println!("oH jeez: ");

}
