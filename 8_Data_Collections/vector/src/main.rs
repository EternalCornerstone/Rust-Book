fn main() {
    let v: Vec<i32> = Vec::new();
    let v1 = vec![1,2,3];
    let mut mut_v = Vec::new();
    mut_v.push(5);
    mut_v.push(6);
    mut_v.push(7);
    mut_v.push(8);
    mut_v.push(9);
    read_vec_alternate(&v);
    read_vec(&v1);
    read_vec(&mut_v);
    doesnt_exist(&v1);
    iteration();
    another_iteration();
    vec_of_enum();
    drop_vec();
}

// will error with an empty vec.
fn read_vec(vec: &Vec<i32>) {
    let third: &i32 = &vec[2];
    println!("the third element is: {third}");
}

fn read_vec_alternate(vec: &Vec<i32>) {
    let third: Option<&i32> = vec.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn doesnt_exist(vec: &Vec<i32>) {
    let doesnt_exist_get = vec.get(100); // this returns None
    println!("returns None if doesn't exist. {doesnt_exist_get:?}");
    // let doesnt_exist_ref = &vec[100]; // this will make the program panic if uncommented
}

fn iteration() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
}
fn another_iteration() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        //dereference variable so that it can be changed
        *i += 50;
    }
    println!("{v:?}");
}
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vec_of_enum() {
    // useful if you need to store multiple types of things within a vec.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{row:?}")
}

fn drop_vec() {
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}