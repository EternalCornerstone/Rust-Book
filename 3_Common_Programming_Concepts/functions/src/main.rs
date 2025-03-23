// comments section added in - simple so doesn't require it's own project.
// // denotes a comment. This is the rust standard.
fn main() {
    println!("Hello, world!");
    let x = five();
    let y = plus_one(5);
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    print_labeled_measurement(x, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}