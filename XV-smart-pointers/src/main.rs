use crate::List::{Cons, Nil};

fn main() {
    I()
}

fn I(){
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

}

enum List{
    Cons(i32, Box<List>),
    Nil
}
