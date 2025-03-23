fn main() {
    println!("Hello, world!");    
    // empty string
    let mut st = String::new();
    
    // to_string on string literal;
    let data = "initial contents";
    let ts = data.to_string();
    
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}

fn hello_in_many_languages() {
    // strings can hold all of these texts because they are utf8 encoded
    let arabic = String::from("السلام عليكم");
    let czech = String::from("Dobrý den");
    let english = String::from("Hello");
    let hebrew = String::from("שלום");
    let sanskrit = String::from("नमस्ते");
    let japanese = String::from("こんにちは");
    let korean = String::from("안녕하세요");
    let chinese = String::from("你好");
    let portuguese = String::from("Olá");
    let russian = String::from("Здравствуйте");
    let spanish = String::from("Hola");
}

// append a string of characters with push_str
fn appending() {
    let mut s = String::from("foo");
    s.push_str("bar");
    
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
}
// append a single character with push
fn appending_push() {
    let mut s = String::from("lo");
    s.push('l');
}

fn concatenation() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}

// manual way to do this.
fn concatenation_of_many() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
}

// correct way with format!() macro
fn concatenation_of_many_correctly () {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
}

// doesn't work
// fn indexing_into_strings() {
//     let s1 = String::from("hello");
//     let h = s1[0];
// }

// strings are wrappers over a Vec<u8>
fn properly_encoded_UTF8() {
    // len is 4 because each letter takes one byte
    let spanish = String::from("Hola");
    // len is 24 - З is cyrillic letter 'Ze' not the number 3
    let russian = String::from("Здравствуйте");
    // this shows why the indexing example doesn't work properly as some "characters" are visual representations of multiple chars
    // let hello = "Здравствуйте";
    // let answer = &hello[0];
    // THIS DOESN'T WORK.
}

// caution greatly if you're creating string slices because doing so can crash the program.

// iterating over strings
fn iteration() {
    // chars iterates over the chars
    for c in "Зд".chars() {
        // returns 
        // З
        // д
        println!("{c}");
    }
    // bytes iterates over the bytes
    for b in "Зд".bytes() {
        // returns
        // 208
        // 151
        // 208
        // 180
        println!("{b}");
    }
}