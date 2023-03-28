use std::ops::Index;

fn main() {
    II();
}

fn II() {

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

    // Return Values and Scope
    {
        let s1 = give_ownership(); // gives return value to s1
        let s2 = String::from("hii"); // s2 comes into scope
        let s3 = take_and_give(s2); // takes s2 and gives back the return value to s3

        // s3 goes out of scope. s2 was moved. s1 goes out of scope

        fn give_ownership() -> String {
            let some_string = String::from("yoo");
            some_string // some_string is moved out into the function that calls it
        }

        fn take_and_give(a_string: String) -> String {
            a_string // a_string is returned and moved out to the calling function
        }
    }
}
