fn main() {
    let s1 = String::from("New string");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");
    println!("Mutable s is: {s}");
    change(&mut s);
    println!("Changed s is: {s}");

    // dangle
    // let reference_to_nothing = dangle();
    let reference_to_string = not_dangling();
}

fn calculate_length(s: &String) -> usize { 
    // s is a reference to a String
    s.len()
} 
// Here, s goes out of scope. But because s does not have ownership of what
// it refers to, the value is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}

// fn example_of_error() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{}, {}, and {}", r1, r2, r3);   
//     // only 1 reference allowed on a data type if that reference is mutable.
// }

fn example_of_working() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
    //this is okay because the reference has been used in the println! and 
    // will not be used again within this scope
}

// //dangling - compiler doesn't like, fix by returning an owned value
// fn dangle() -> &String { 
//     // dangle returns a reference to a String
//     let s = String::from("hello"); 
//     // s is a new String

//     &s 
//     // we return a reference to the String, s
// } 
// Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!
fn not_dangling() -> String {
    let s = String::from("hello");
    s
}