fn main() {
    println!("Panic stations commence!");
    panic_from_necessity();
}

fn panic_from_necessity() {
    let v = vec![1, 2, 3];

    v[99];
}

fn panic_from_choice() {
    panic!("Crash and burn!");
}