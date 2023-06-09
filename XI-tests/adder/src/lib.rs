extern crate core;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larg_can_hold_small(){
        let larger = Rectangle{
            width: 8,
            height: 7
        };
        let smaller = Rectangle{ width: 5, height: 1 };
        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn larg_can_not_hold_small(){
        let larger = Rectangle{
            width: 8,
            height: 7
        };
        let smaller = Rectangle{ width: 5, height: 1 };
        let result = smaller.can_hold(&larger);
        assert!(
            !result,
            "the return of this fn is {}",
            result
        )
    }

    #[test]
    #[should_panic(expected = "index")]
    fn out_of_bound(){
        let list = vec![1,2,3];
        list[3];
    }
    #[test]
    #[should_panic]
    #[ignore]
    fn another(){
        panic!("it failed")
    }

    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
