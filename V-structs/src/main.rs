fn main() {
    I()
}

fn I() {
    struct User{
        active: bool,
        username: String,
        gmail: String,
        sign_in_count: u64,
    }

    let mut user = User {
        active: true,
        username: String::from("mito"),
        gmail: String::from("hi@gmail.com"),
        sign_in_count: 3
    };

    println!("{ }", user.gmail);
    user.gmail = String::from("hello@gmail.com");
    println!("{ }", user.gmail);

}
