use X_generics_traits_lifetime::{
    notify, notify2, notify3, notify4, return_summary, NewsArticle, Summary, Tweet,
};

fn main() {


    //IV();
}

fn IV() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    fn main() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result =
            longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
        println!("The longest string is {}", result);
    }

    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

fn III() {
    let tweet = Tweet {
        username: "mito".to_string(),
        content: "i hate this world".to_string(),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: "".to_string(),
        location: "".to_string(),
        author: "".to_string(),
        content: "".to_string(),
    };
    let int = 15;

    dbg!(notify4(&int));
    dbg!(notify(&tweet));
    dbg!(notify2(&article));

    println!("{ }", tweet.summarize());
    println!("{ }", tweet.summarize_author());
    println!("{ }", article.summarize());

    dbg!(return_summary().summarize());
}

fn II() {
    #[derive(Debug)]
    struct Dor<X1, Y1> {
        x: X1,
        y: Y1,
    }
    impl<X1, Y1> Dor<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Dor<X2, Y2>) -> Dor<X1, Y2> {
            Dor {
                x: self.x,
                y: other.y,
            }
        }
    }

    let d1 = Dor { x: 5, y: 10.4 };
    let d2 = Dor { x: "mito", y: 'c' };

    dbg!(&d1.mixup(d2));

    //-------------------

    enum Hix<T> {
        Yoo(T),
        Hillo(T),
    }

    let x = Hix::Hillo("");

    match x {
        Hix::Yoo(_) => {}
        Hix::Hillo(_) => {}
    }
    //---------------
    struct Point<T> {
        x: T,
        y: T,
    }
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let point: Point<i32> = Point { x: 5, y: 10 };
    let x = point.x();
    let y = point.y;

    println!("{} {}", x, y);

    let fpoint: Point<f32> = Point { x: 1.34, y: 2.2 };
    fpoint.distance_from_origin();

    //-----------
    let char_list = vec!['y', 'm', 'z', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    fn largest<T: PartialOrd>(number_list: &[T]) -> &T {
        let mut largest = &number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }
        largest
    }
}

fn I() {
    let number_list1 = vec![34, 50, 25, 100, 65];

    let number_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("The largest number is {}", largest(&number_list1));
    println!("The largest number is {}", largest(&number_list2));

    fn largest(number_list: &[i32]) -> &i32 {
        let mut largest = &number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        largest
    }
}
