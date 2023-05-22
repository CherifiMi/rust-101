use std::thread;
use std::time::Duration;

fn main() {
    IV()
}


fn III() {

    /// doesnt borrow
    /// ```kotlin
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
    /// ```kotlin
    /// let mut list = vec![1,2,3];
    /// println!("before closure: {:?}", list);
    /// let mut borrows_mutably  = || list.push(5);
    /// borrows_mutably();
    /// println!("before closure: {:?}", list);
    /// ```

    let list = vec![1,2,3];
    println!("before closure: {:?}", list);

    thread::spawn(move || println!("from thread: {:?}", list))
        .join()
        .unwrap();

}

fn II() {
    let expansive_closure = |num: u32| -> u32{
        println!("calcilating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    //fn add_one1(x:u32) -> u32 { x + 1 }
    //let add_one2 = |x:u32| -> u32 { x + 1 };
    //let add_one3 = |x| { x + 1 };
    let add_one4 = |x| x+1;


    let r_c = |x| x;

    dbg!(add_one4(5));
    dbg!(expansive_closure(12));
    dbg!(r_c(14));
    //dbg!(r_c(String::from("hillo"))); wont work
}

fn I() {
    let store = Inventory { shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red] };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("the user with pref {:?} gets {:?}",user_pref1, giveaway1);

    let user_pref2 = Some(ShirtColor::Blue);
    let giveaway2 = store.giveaway(user_pref2);
    println!("the user2 with pref {:?} gets {:?}",user_pref2, giveaway2);
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