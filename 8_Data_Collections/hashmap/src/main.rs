// requires using std::collections::HashMap - not in the prelude
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    create_new_hashmap();
    ownership_in_hashmap();
    overwriting_hashmap_data();
    check_for_existing();
    lookup_and_update_old();
}

fn create_new_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue has scored: {score}");
    for (key, value) in &scores {
        println!("{key}: {value}");
    };
}

fn ownership_in_hashmap() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}

fn overwriting_hashmap_data() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    // This code will print {"Blue": 25}. The original value of 10 has been overwritten.
    println!("{scores:?}");
}

// .entry checks a key as a parameter and returns an Entry. or_insert responds
// to empty entry value and inserts a row with the key + value within the or_insert parameter
fn check_for_existing() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}

fn lookup_and_update_old() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

fn hashing_function() {
    // By default, HashMap uses a hashing function called SipHash 
    // that can provide resistance to denial-of-service (DoS) attacks 
    // involving hash tables1. This is not the fastest hashing algorithm available, 
    // but the trade-off for better security that comes with the drop in performance
    // is worth it. If you profile your code and find that the default hash function 
    // is too slow for your purposes, you can switch to another function by specifying 
    // a different hasher. A hasher is a type that implements the BuildHasher trait. 
    // We’ll talk about traits and how to implement them in Chapter 10. 
    // You don’t necessarily have to implement your own hasher from scratch; 
    // crates.io has libraries shared by other Rust users that provide hashers 
    // implementing many common hashing algorithms.
}