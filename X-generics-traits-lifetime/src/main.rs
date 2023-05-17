
fn main() {
    III();
}

fn III() {//
}

fn II(){
    #[derive(Debug)]
    struct Dor<X1,Y1>{
        x: X1,
        y: Y1
    }
    impl <X1,Y1> Dor<X1, Y1>{
        fn mixup<X2,Y2>(self, other: Dor<X2,Y2>) -> Dor<X1,Y2>{
            Dor{
                x:self.x,
                y:other.y
            }
        }
    }

    let d1 = Dor{x: 5, y: 10.4};
    let d2 = Dor{x: "mito", y: 'c'};

    dbg!(&d1.mixup(d2));

    //-------------------

    enum Hix<T>{
        Yoo(T),
        Hillo(T)
    }

    let x = Hix::Hillo("");

    match x {
        Hix::Yoo(_) => {}
        Hix::Hillo(_) => {}
    }
    //---------------
    struct Point<T>{
        x:T,
        y:T
    }
    impl <T> Point<T>{
        fn x(&self) -> &T{
            &self.x
        }
    }
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let point:Point<i32> = Point{x: 5, y: 10};
    let x = point.x();
    let y = point.y;


    println!("{} {}",x,y);

    let fpoint: Point<f32> = Point{x:1.34,y : 2.2};
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
