use std::thread::{self , JoinHandle};
use std::time::Duration;

fn msg_hello() -> &'static str {
    thread::sleep(Duration::from_secs(2));
    "Hello"
} 

fn msg_world() -> &'static str {
    thread::sleep(Duration::from_secs(1));
    "world"
}

fn msg_final() -> &'static str {
    thread::sleep(Duration::from_secs(1));
    "!"
}

fn main() {
    let a = thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        50
    });

    println!("Waiting for data...");
    match a.join() {
        Ok(value) => println!("{}", value),
        Err(err) => println!("{:?}", err),
    };

    let j1 = thread::spawn( move || msg_hello());
    let j2 = thread::spawn( move || msg_world());
    let j3 = thread::spawn( move || msg_final());

    let s1 = j1.join().expect("Failed to get message");
    let s2 = j2.join().expect("Failed to get message");
    let s3 = j3.join().expect("Failed to get message");

    println!("{} {}{}", s1, s2, s3);
}
