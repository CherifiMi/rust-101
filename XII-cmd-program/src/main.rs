use std::{env, fs, process};
use std::error::Error;
use XII_cmd_program::Config;

fn main() {
    /*let args: Vec<String> = env::args().collect();

    let config =Config::build(&args).unwrap_or_else(|err|{
        println!("problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = config.run(){
        println!("problem running: {}", e);
        process::exit(1);
    }*/

    let burg = Burgur
    ::new("borgaor with coca cola".to_string())
        .setPatty("halal meat".to_string())
        .setCheese("lavach kiri".to_string())
        .setBun("baryoch".to_string())
        .add("coca cola".to_string())
        .add("ice".to_string());

    dbg!(&burg);
}

#[derive(Debug)]
struct Burgur {
    name: String,
    bun: String,
    patty: String,
    cheese: String,
    adding: Vec<String>
}
impl Burgur {
    fn new(name: String) -> Burgur{
        Burgur {
            name,
            bun: "".to_string(),
            patty: "".to_string(),
            cheese: "".to_string(),
            adding: Vec::new(),
        }
    }
    fn setPatty(&mut self, s: String) -> &mut Burgur {
        self.patty = s;
        self
    }
    fn setBun(&mut self, s: String) -> &mut Burgur {
        self.bun = s;
        self
    }
    fn setCheese(&mut self, s: String) -> &mut Burgur {
        self.cheese = s;
        self
    }
    fn add(&mut self, v: String)-> &mut Burgur{
        self.adding.push(v);
        self
    }
}