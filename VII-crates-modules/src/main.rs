use crate::garden::fruit::Banana;
use crate::garden::say_hi;
use crate::jnan::fruit2::Banana2;
use crate::waten::fruit3::Dela3;

pub mod garden{
    pub fn say_hi(){
        println!("hilllllo")
    }

    pub mod fruit {
        #[derive(Debug)]
        pub struct Banana{}
    }
}

pub mod jnan;
pub mod waten;

fn main() {
    let plant1 = Banana{};
    let plant2 = Banana2{};
    let plant3 = Dela3{};
    println!("i am growing {:?}", plant3);
    say_hi()
}
