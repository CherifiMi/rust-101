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
}