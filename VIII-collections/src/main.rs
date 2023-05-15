use std::arch::x86_64::_blci_u32;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use std::io::stdin;
use std::str::SplitWhitespace;

fn main() {
    ex3()
}

//region ex


/*
- ex1: Given a list of integers, use a vector and return the median (when sorted, the value in
    the middle position) and mode (the value that occurs most often;
    a hash map will be helpful here) of the list.

- ex2: Convert strings to pig latin. The first consonant of each word
    is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
    Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
    Keep in mind the details about UTF-8 encoding!

- ex3: Using a hash map and vectors, create a text interface to
    allow a user to add employee names to a department in a company.
     For example, “Add Sally to Engineering” or “Add Amir to Sales.”
     Then let the user retrieve a list of all people in a department or all people in
     the company by department, sorted alphabetically.
*/

fn ex3() {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    loop {
        println!("enter command:");
        let mut s = String::new();
        stdin().read_line(&mut s).expect("");

        let list: Vec<&str> = s.split(' ').collect();

        let instruction = list[0];
        let order1 = list[1];
        let order2 = list[3];

        if instruction == "add" {
            let worker = map.entry(order2).or_insert(vec![]);
            worker.push(order1);

            println!("{} has been added to {}", order1, order2);
        }
        else if instruction == "get" {
            if order1=="all" {
                dbg!(&map);
            }
            else {
                dbg!(&map.get(order1));
            }
        }
        else {
            println!("something went wrong")
        }
    }

}

fn ex2() {
    let s = String::from("mito");

    let latin_s = {
        let vols = ["a","o","i","u","e"];
        let c1 = &s[0..1];
        let rest = &s[1..s.len()];

        if vols.contains(&c1){
            format!("{s}_hay")
        }else {
            format!("{rest}_{c1}ay")
        }

    };

    dbg!(latin_s);
}

fn ex1() {
    let mut list = vec![0, 2, 12, 123, 21, 21, 22, 22, 1, 22, 2, 34, 54, 88, 9, 0, 0];


    let mut median  = {
        let mut v = list.clone();
        v.sort();
        v[list.len()/2]
    };

    let mode = {
        let mut map = HashMap::new();
        for i in &list{
            let count = map.entry(i).or_insert(0);
            *count+=1;
        }
        map
    };

    dbg!(median);
    dbg!(mode);
}
//endregion

fn HashMaps() {
    let mut scores = HashMap::new();

    scores.insert("blue", 20);
    scores.insert("red", 40);
    scores.insert("red", 60);

    scores.entry("blue").or_insert(21);
    scores.entry("yellow").or_insert(233);

    let blue_team = scores.get("blue").copied().unwrap_or(0);
    dbg!(blue_team);

    for(key, value) in &mut scores{
        *value += 5;
        println!("{key}: {value}");
    }

    //___________

    let text = "mito is my name my name i mito the mito";
    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let mut count = map.entry(word).or_insert(0);
        *count+=1;
        dbg!(word, count);
    }
}

fn StringsFn(){
    let s = "my name is mito ";

    let word = |l: i32| -> &str {
        let mut r = (99, 99);
        let mut space_count = 0;

        for (i, &c) in s.as_bytes().iter().enumerate() {
            if c == b' ' {
                space_count+=1;
                println!("{i}");
            }
            if space_count == l-1 && r.0 == 99 { r.0 = i }
            if space_count == l && r.1 == 99 { r.1 = i}
        }
        &s[r.0..r.1]
    };

    let s1 = word(1);
    let s2 = word(2);
    let s3 = word(3);
    let s4 = word(4);

    println!("{s1}");
    println!("{s2}");
    println!("{s3}");
    println!("{s4}");
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
