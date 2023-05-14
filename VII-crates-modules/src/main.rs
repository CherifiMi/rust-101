use crate::garden::fruit::Banana;
use crate::garden::say_hi;

pub mod garden{
    pub fn say_hi(){
        println!("hilllllo")
    }


    pub mod fruit {
        #[derive(Debug)]
        pub struct Banana{}
    }
}

fn main() {
    let plant = Banana{};
    println!("i am growing {:?}", plant);
    say_hi()
}
