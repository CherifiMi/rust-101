fn main() {
    Vectors()
}

fn Vectors() {
    let mut v: Vec<i32> = Vec::new();
    let mut v2 = vec![1,2,3];

    v.push(2);
    v.push(7);
    v.push(8);
    v.push(99);

    let second: &i32 = &v[1];
    println!("second {}", second);

    let third: Option<&i32> = v.get(3);
    match third {
        None => {println!("there is no third")}
        Some(third) => {println!("third is {}", third)}
    }


    let first_v2 = &v2[0];
    v2.push(5);
    // barrow bug println!("the first elemnt {}", first_v2)


    for i in v {
        println!("{i}");
    }
}
