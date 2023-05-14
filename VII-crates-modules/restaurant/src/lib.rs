pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist()
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("hillo")
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
