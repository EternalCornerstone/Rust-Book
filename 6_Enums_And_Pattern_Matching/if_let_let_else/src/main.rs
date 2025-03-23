#[derive(Debug)]
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

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

fn main() {
    println!("Hello, world!");
    match_option_sloppy();
    better_syntax();
    if let Some(description) = describe_state_quarter_better(Coin::Quarter(UsState::Alabama)) {
        println!("{}", description);
    }
}

fn match_option_sloppy(){
    let config_max = Some(3u8);
    match config_max {
        Some(max) => { 
            println!("The maximum is configured to be {max}")
        }
        _ => ()
    };
}
// think of if let as syntax sugar for a match that runs code when 
// the value matches one pattern and then ignores all other values.
fn better_syntax() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}


fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn describe_state_quarter_alternate(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_quarter_better(coin: Coin) -> Option<String> {
    println!("inside describe state {coin:?}");
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}