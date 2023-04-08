use std::net::Shutdown::Write;
use std::string::FromUtf8Error;

fn main() {
    I();
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
        enum Message{
            Quit,
            Move{x:i32, y: i32},
            Write(String),
            ChangeColor(i32,i32,i32)
        }
        struct QuitMessage;
        struct MoveMessage{
            x: i32,
            y: i32
        }
        struct WriteMessage(String);
        struct ChangeColor(i32,i32,i32);

        impl Message{
            fn call(&self){

                dbg!(self);
            }
        }

        let m = Message::Write(String::from("hillo"));
        m.call()
    }
}
