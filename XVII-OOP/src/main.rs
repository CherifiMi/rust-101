use std::thread::scope;

fn main() {
    let S = Screens { components: vec![
        Box::new(
            Button{
                width: 10,
                height: 2,
                label: String::from("hi")
            }
        ),
        Box::new(
            Button{
                width: 10,
                height: 2,
                label: String::from("ho")
            }
        ),
    ]};

    S.run()
}

trait Draw {
    fn draw(&self);
}
struct Screens {
    components: Vec<Box<dyn Draw>>,
}
impl Screens {
    fn run(&self) {
        for c in self.components.iter() {
            c.draw()
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{}", self.label);
    }
}
