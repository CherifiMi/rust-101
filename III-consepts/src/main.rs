fn main() {
    II()
}

fn II() {
    // integer
    let a = 99_233;
    println!("{}", a);
    let a = 0xfd;
    println!("{}", a);
    let a = 0o76;
    println!("{}", a);
    let a = 0b_1110_1011;
    println!("{}", a);
    let a = b'g';
    println!("{}", a);

    // float
    let a = 2.0;
    let a:f32 = 3.0;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // char
    let c = 'z';
    let skull = '☠';
    println!("{}", skull);

}

fn I(){
    let mut x = 5;
    println!("{}", { x });
    x = 6;
    println!("{}", { x });

    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("{}", THREE_HOURS_IN_SECONDS);

    {
        let x = x.to_string();
        println!("{}", { x });
    }

    println!("{}", { x });
}
