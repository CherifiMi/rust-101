use std::io::Read;

fn main() {
    III()
}

fn III() {
    #[derive(Debug)]
    struct Rect{
        width: u32,
        height: u32
    }
    impl Rect{
        fn area(&self) -> u32 {
            self.width*self.height
        }
        fn width(&self) -> bool{
            self.width>20
        }
        fn can_hold(&self,rect: &Rect) -> bool{
            self.area()>=rect.area()
        }
    }
    impl Rect{
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let rect = Rect{ width: 50, height: 30 };
    let rec1 = Rect{ width: 60, height: 40 };
    let rec2 = Rect{ width: 50, height: 20 };

    let care = Rect::square(10);

    dbg!(care);

    println!("{ }", rect.area());
    println!("{ }", rect.width());

    println!("{ }", rect.can_hold(&rec1));
    println!("{ }", rect.can_hold(&rec2));
}

fn II() {
    {
        let width1 = 30;
        let height1 = 50;

        println!("{ }", area(width1, height1));

        fn area(width: u32, height: u32) -> u32 {
            width * height
        }
    }
    {
        let rect = (30, 50);
        println!("{ }", area(rect));

        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0*dimensions.1
        }
    }
    {
        #[derive(Debug)]
        struct Rect {
            width: u32,
            height: u32
        }

        let rect1 = Rect{
            width: 30,
            height: 30,
        };

        dbg!(&rect1);

        println!("rect1 is {:#?}", rect1);
        println!("{ }", area(&rect1));

        fn area(rect: &Rect) -> u32 {
            rect.width*rect.width
        }

    }
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

        dbg!(&rect1);
    }
}

fn I() {
    {
        struct User {
            active: bool,
            username: String,
            gmail: String,
            sign_in_count: u64,
        }

        let mut user = User {
            active: true,
            username: String::from("mito"),
            gmail: String::from("hi@gmail.com"),
            sign_in_count: 3,
        };

        let user1 = build_user(
            String::from("mito@gmail"),
            String::from("the mito"),
        );

        let user2 = User {
            active: true,
            gmail: user1.gmail,
            username: user.username,
            sign_in_count: 10,
        };

        let user4 = User {
            username: String::from("hillo"),
            ..user2
        };

        user.gmail = String::from("hello@gmail.com");

        println!("{ }", user4.username);

        fn build_user(gmail: String, username: String) -> User {
            User {
                active: true,
                username,
                gmail,
                sign_in_count: 1,
            }
        }}

    {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }

    {
        struct Always_true;

        let subject = Always_true;
    }
}
