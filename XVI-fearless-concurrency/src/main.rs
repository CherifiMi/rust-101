use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handels = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handel = thread::spawn(move||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handels.push(handel);
    }

    for handle in handels{
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());
}