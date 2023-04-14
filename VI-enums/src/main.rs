use std::net::Shutdown::Write;
use std::string::FromUtf8Error;

fn main() {
    III();
}

fn I() {
    {
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

        enum IpAddr2 {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home = IpAddr {
            kind: IpAddKind::V4,
            address: String::from("192.111"),
        };

        let loopback = IpAddr {
            kind: six,
            address: String::from("::1"),
        };

        let yoo = IpAddr2::V6(String::from("yoo"));
        let boo = IpAddr2::V4(123, 33, 0, 0);

        route(four);

        fn route(ip_kind: IpAddKind) -> IpAddKind {
            ip_kind
        }
    }
    {
        struct Ipv4Addr {
            // --snip--
        }

        struct Ipv6Addr {
            // --snip--
        }

        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
    }
    {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        impl Message {
            fn call(&self) {
                match self {
                    Message::Quit => {
                        println!("quit")
                    }
                    Message::Move {x,y} => {
                        println!("move {} {}", x, y)
                    }
                    Message::Write(s) => {
                        println!("write { }", s)
                    }
                    Message::ChangeColor(a, b, c) => {
                        println!("color {}", a + b + c)
                    }
                }
            }
        }

        struct QuitMessage;
        struct MoveMessage {
            x: i32,
            y: i32,
        }
        struct WriteMessage(String);
        struct ChangeColor(i32, i32, i32);

        let w = Message::Write(String::from("hillo"));
        let c = Message::ChangeColor(3, 43, 2);
        let m = Message::Move {x:5, y: 9 };
        let q = Message::Quit;

        w.call();
        c.call();
        m.call();
        q.call()
    }
    {
        let s = Some(5);
        let x: Option<i32> = None;

        match x {
            None => {println!("null")}
            Some(it) => {println!("{}", it)}
        }
    }
}
fn II(){
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        println!("{}", value_in_cents(Coin::Nickel));
        value_in_cents2(Coin::Penny);

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
        fn value_in_cents2(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("Lucky penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
    }
    {
        #[derive(Debug)]
        enum UsState{
            Alabama,
            Alaska,
        }
        enum Coin{
            Penny,
            Nickel,
            Dime,
            Quarter(UsState)
        }

        value_in_cents(Coin::Quarter(UsState::Alaska));

        fn value_in_cents(coin: Coin) -> u8{
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }
    }
    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        dbg!("{}", six);
        dbg!("{}", none);

        fn plus_one(x: Option<i32>)-> Option<i32>{
            match x {
                None => None,
                Some(i) => Some(i+1)
            }
        }
    }
    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),    // use all other possibility
            _ => move_one(),    // to all other  possibility, call the same fn
            _ => (),            // to all other  possibility, do nothing
        }

        fn move_one(){}
        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn move_player(num_spaces: u8) {}
    }
}
fn III(){

}
