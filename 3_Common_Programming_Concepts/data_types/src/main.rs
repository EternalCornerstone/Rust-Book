use std::io;
fn main() {
    // math
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;
    
    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // char - uses single quotes not double quotes
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // retrieve tuple value (destructuring)
    let (x, y, z) = tup;
    println!("The value of x is: {x}, y is: {y}, z is: {z}");
    //alternate method of accessing tuple values
    let t: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = t.0;
    let six_point_four = t.1;
    let one = t.2;

    // arrays and vectors  
    // arrays are fixed, vectors can grow and shrink
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // semi colon to denote # of elements
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"]; 
    // array is logical here because the months are a static data set
    let b = [3; 5]; // array of 5 elements initialised as 3's ([3,3,3,3,3])
    let first = b[0];
    let second = b[1];

    //array errors
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
