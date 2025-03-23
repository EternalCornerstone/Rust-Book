enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}
#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match &self {
            Message::Write(v) => println!("The message is {}", v),
            Message::Quit => println!("user chose to quit"),
            Message::ChangeColor(r, g, b) => println!("User chose to change color to {}, {}, {}", r, g, b),
            Message::Move {x, y} => println!("moving to value: x {}, y {}", x, y),
            _ => println!("nothing"),
        };
    }
}

fn option_use() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Hello, world!");
    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    let write = Message::Write(String::from("hello"));
    write.call();
    let quit = Message::Quit;
    quit.call();
    let mv = Message::Move {x: 12, y: 15};
    mv.call();
    let change_color = Message::ChangeColor(15, 12, 25);
    change_color.call();
}

// fn route(kind: IpAddrKind) {

// }