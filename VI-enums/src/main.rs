use std::string::FromUtf8Error;

fn main() {
    //I();

    x();

}

fn I() {
    enum IpAddKind {
        V4,
        V6,
    }

    let four = IpAddKind::V4;
    let six = IpAddKind::V6;

    struct IpAddr {
        kind: IpAddKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("192.111"),
    };

    let loopback = IpAddr {
        kind: six,
        address: String::from("::1"),
    };

    route(four);

    fn route(ip_kind: IpAddKind) -> IpAddKind {
        ip_kind
    }
}

fn x(){

    let mut s = String::from("hello world");
    let first_index = first_word(&s);
    let first_word = index_to_word(first_index, &s);

    println!("{ }", first_word);

    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        s.len()
    }


    fn index_to_word(word: usize, s: &String) -> String {
        let mut v: Vec<u8> = Vec::new();
        for i in 0..word {
            v.push(s.as_bytes()[i]);
        }
        let s = String::from_utf8(v);

        return match s {
            Ok(n) => { n }
            Err(_) => { String::new() }
        }
    }
}
