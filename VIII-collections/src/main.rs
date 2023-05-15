fn main() {
    Strings()
}

fn Strings() {
    let data = "starting data".to_string();
    let data2 = String::from("starting data");
    let mut s = String::new();

    let mut hello = String::from("السلام عليكم");
    hello.push_str(" means peace be upon you");
    println!("{hello}");
    s.push_str(&hello);
    println!("{hello}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let ss = format!("{s3} is {s2}");

    let hil = &hello[0..4].to_string();
    println!("{ }", &hil);

    for i in "السلام عليكم".chars(){
        println!("{} ", i)
    }
    for i in "السلام عليكم".bytes() {
        println!("{}", i)
    }
}

fn Vectors() {

    let mut v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    v.push(2);
    v.push(7);
    v.push(8);
    v.push(99);

    let second: &i32 = &v[1];
    println!("second {}", second);

    let third: Option<&i32> = v.get(3);
    match third {
        None => { println!("there is no third") }
        Some(third) => { println!("third is {}", third) }
    }

    let first_v2 = &v2[0];
    v2.push(5);
    // barrow bug println!("the first elemnt {}", first_v2)


    for i in v {
        println!("{i}");
    }


    for i in &mut v2 {
        *i += 9;
    }

    for i in v2 {
        println!("{i}");
    }

    //_______
    #[derive(Debug)]
    enum Sheet {
        Num(i32),
        Text(String),
        On(bool),
    }

    let calendar = vec![Sheet::Num(2003), Sheet::On(true), Sheet::Text(String::from("mito birthday")), Sheet::Num(15)];
    for i in calendar {
        println!("{:#?}", i);
        match i {
            Sheet::Num(_) => {}
            Sheet::Text(_) => {}
            Sheet::On(_) => {}
        }
    }
}
