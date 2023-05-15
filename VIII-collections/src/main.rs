fn main() {
    Strings()
}

fn Strings() {
    let data = "starting data".to_string();
    let s = String::new();
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
