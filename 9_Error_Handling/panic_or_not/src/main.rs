// generally panic for prototyping and testing, else return a result.
use std::net::IpAddr;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
    // getter
    pub fn value(&self) -> i32 {
        self.value
    }
}
fn main() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
        let g = Guess::new(99);
        let v  = g.value();
        println!("{v}");
}
