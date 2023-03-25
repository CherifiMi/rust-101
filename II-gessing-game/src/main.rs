use std::cmp::Ordering;
use std::io;

fn main() {

    println!("Guess the number");
    let rand = 29;
    loop {
        let mut geuss = String::new();
        io::stdin()
            .read_line(&mut geuss)
            .expect("fail to read line");

        let geuss: u32 =match geuss
            .trim()
            .parse()
            {
                Ok(num) => num,
                Err(_)=>continue,
            };

        println!("you gessed {geuss}");

        match geuss.cmp(&rand) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("win");
                break;
            },
        }
    }
}
