use std::thread;
use std::time::Duration;

fn main() {

    let job = thread::spawn(||{
        for x in 1..10 {
            println!("hi num is {}", x);
            thread::sleep(Duration::from_millis(1));
        }
    });

    job.join().unwrap();

    for x in 1..5 {
        println!("hi num from main {}", x);
        thread::sleep(Duration::from_millis(1))
    }

}
