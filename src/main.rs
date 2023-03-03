use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];
    let copied = v.clone();
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread! {:?}", i, copied);
            thread::sleep(Duration::from_millis(1));
        }
    });

    println!("Hello, world! {:?}", v);

    handle.join().unwrap();
    println!("Hello, world!");

    println!("Hello, world!");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}