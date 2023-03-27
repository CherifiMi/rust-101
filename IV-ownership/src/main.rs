use std::ops::Index;

fn main() {
    I();
}

fn I() {
    {       // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        // do stuff with s
    }       // this scope is now over, and s is no longer valid


    let x = ", world";
    let mut s = String::from("hii");
    s.push_str(x);
    println!("{}",s);

    let s2 = s.clone();

    // s is dead
    for i in s.as_bytes(){
        println!("{}",i);
    }

    // functions and scope
    {
        let s = String::from("hello");  // s comes into scope

        takes_ownership(s);             // s's value moves into the function...
        // ... and so is no longer valid here

        let x = 5;                      // x comes into scope

        makes_copy(x);                  // x would move into the function,
        // but i32 is Copy, so it's okay to still
        // use x afterward

    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.

}