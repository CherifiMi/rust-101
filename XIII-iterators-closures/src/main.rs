use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use std::ptr::hash;
use std::thread;
use std::time::Duration;

fn main() {

    let mut map = HashMap::new();

    map.insert(3, "fizz");
    map.insert(5, "buzz");
    map.insert(7, "mopp");

    for i in 1..1000 {
        let mut result = String::new();
        for x in map.keys() {
            if i%x == 0 {
                result.push_str(map[x])
            }
        }
        if result.is_empty() {
            result.push_str(i.to_string().as_str())
        }
        println!("{}", result)
    }

    //IV()
}

fn IV() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();


    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None)
}

fn III() {
    /// doesnt borrow
    /// ```rust
    /// let list = vec![1,2,3];
    /// println!("before closure: {:?}", list);
    ///
    /// let only_borrow = || println!("from closure {:?}", list);
    ///
    /// println!("before calling closure: {:?}", list);
    /// only_borrow();
    /// println!("before closure: {:?}", list);
    /// ```
    /// mut closure
    /// ```rust
    /// let mut list = vec![1,2,3];
    /// println!("before closure: {:?}", list);
    /// let mut borrows_mutably  = || list.push(5);
    /// borrows_mutably();
    /// println!("before closure: {:?}", list);
    /// ```
    let list = vec![1, 2, 3];
    println!("before closure: {:?}", list);

    thread::spawn(move || println!("from thread: {:?}", list))
        .join()
        .unwrap();
}

fn II() {
    let expansive_closure = |num: u32| -> u32 {
        println!("calcilating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    //fn add_one1(x:u32) -> u32 { x + 1 }
    //let add_one2 = |x:u32| -> u32 { x + 1 };
    //let add_one3 = |x| { x + 1 };
    let add_one4 = |x| x + 1;

    let r_c = |x| x;

    dbg!(add_one4(5));
    dbg!(expansive_closure(12));
    dbg!(r_c(14));
    //dbg!(r_c(String::from("hillo"))); wont work
}

fn I() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("the user with pref {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = Some(ShirtColor::Blue);
    let giveaway2 = store.giveaway(user_pref2);
    println!("the user2 with pref {:?} gets {:?}", user_pref2, giveaway2);
}

// region closure 1
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

// endregion
