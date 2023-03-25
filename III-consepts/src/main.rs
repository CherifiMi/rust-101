fn main() {
    V()
}

fn V() {
    // conditions
    {
        let number = 3;

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }


        let condition = true;
        let number = if condition { 5 } else { 6 };


        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
    }
}
    // loops
    {
        let mut i = 0;
        loop {
            i+=1;
            println!("hi {}", i);
            if i==20 {break}
            if i!=10 {continue}
            println!("fuck yyy");
        }


        let mut counter = 0;
        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter*5;
            }
        };
        println!("{result}");


        let mut count = 0;
        'mito: loop {
            println!("count = {count}");
            let mut remaining = 10;
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'mito;
                }
                remaining -= 1;
            }
            count += 1;
        }
        println!("End count = {count}");


        let mut number = 3;
        while number != 0 {
            println!("{number}!");
            number -= 1;
        }
        println!("LIFTOFF!!!");


        let a = [10, 20, 30, 40, 50];

        for element in a {
            println!("the value is: {element}");
        }
    }
}

fn IV() {
    // this is a comment
    /* this is a block comment*/
}

fn III() {

    input(2, 'k');

    println!("{}", rtrn(true));

    fn input(x: i32, c: char) {
        println!("The value of x is: {}", x);
    }

    fn rtrn(x: bool) -> String {
        x.to_string()
    }

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

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

    // tuple
    let tup = (50, true, "hi");
    let tup:(i32,f32,bool)=(5,5.5,false);

    let (x,y,z) = tup;
    println!("{}", tup.0);
    println!("{}", z);

    // array

    let a: [i32;5] = [1, 2, 3, 4, 5];

    let a = [3,4,32,45,234,1,2];
    a.len();

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let b = [6;7];
    for p in a {
        print!("{} ",p)
    }

    println!("{}",months[3]);
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
