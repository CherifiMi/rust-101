use std::borrow::Borrow;
use std::fmt::Debug;
use std::intrinsics::write_bytes;
use std::ops::Index;

fn main() {
    III();
}

fn III() {
    {
        let mut s = String::from("hello world");
        let first = first_word(&s);
        let second = second_word(&s);

        //println!("first word is { }", &s[0..first_word(&s)]);
        //println!("second word is{ }", &s[second.0..second.1]);

        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i;
                }
            }
            s.len()
        }
        fn second_word(s: &String) -> (usize, usize) {
            let mut start = 0;
            let mut end = 0;

            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    if start == 0 {
                        start = i;
                        continue;
                    }
                    if start != 0 && end == 0 {
                        end = i;
                    }
                }
            }
            end = s.len();
            (start, end)
        }
        /*fn index_to_word(word: usize, s: &String) -> String {
            let mut v: Vec<u8> = Vec::new();
            for i in 0..word {
                v.push(s.as_bytes()[i]);
            }
            let s = String::from_utf8(v);

            s
        }*/
    }
    // string slice
    {
        let s = String::from("We get the index for the end");

        let hello = &s[..5]; // 0..5 is the same as ..5
        let world = &s[6..]; //6..s.len() same as 6..
        let all = &s[..]; // same as &s

        println!("first word is { }",first_word(&s));
        println!("second word is { }",second_word(&s));

        fn first_word(s: &str) -> &str { // &str allows both str and string
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[..i];
                }
            }
            &s[..]
        }
        fn second_word(s: &String) -> &str {
            let bytes = s.as_bytes();
            let mut x = (0,s.len());

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    if x.0 != 0 {
                        x.1 = i;
                        break
                    }
                    x.0 = i+1;
                }
            }

            &s[x.0..x.1]
        }
    }

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    for i in &[2, 3] {
        println!("{ }",i);
    }
    for i in slice {
        println!("{ }",i);
    }
}

fn II() {
    // References
    {
        let s1 = String::from("hi");
        let len = calculate_lin(&s1);
        println!("{} {}", s1, len);
        fn calculate_lin(s: &String) -> usize {
            s.len()
        }
    }

    // Mutable References
    {
        // you can only have one mutable reference
        let mut s = String::from("hello");

        {
            let s1 = &mut s;
        } // s1 goes out of scope

        change(&mut s);

        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }
    }
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
