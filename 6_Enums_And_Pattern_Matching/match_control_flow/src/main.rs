enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

fn main() {
    // matching Enums
    let penny = value_in_cents(Coin::Penny);
    let nickel = value_in_cents(Coin::Nickel);
    let dime = value_in_cents(Coin::Dime);
    let quarter = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("a penny is worth {penny} cent!");
    println!("a nickel is worth {nickel} cents!");
    println!("a dime is worth {dime} cents!");
    println!("a quarter is worth {quarter} cents!");

    // matching Options - Some<T> / None
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five variable is: {five:?}, six is: {six:?}, none is {none:?}");

    // matching with a catch all
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
