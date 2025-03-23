fn main() {
    let s = "String test";
    let mut st = String::from("String test");

    let word = first_word(&s); // word will get the value 5
    let second_word = second_word(&s);

    // `word` still has the value `5` here, but `s` no longer has any content
    // that we could meaningfully use with the value `5`, so `word` is now
    // totally invalid!

    println!("returned first word is: '{word}'");
    println!("returned second word is: '{second_word}'");
}
fn slice_syntax() {
    let s = String::from("hello");

    let slice_a = &s[0..2]; //specific syntax
    let slice_b= &s[..2];  // don't need to specify index if you want it from 0

    //case for end of string
    let len = s.len();
    let slice_c = &s[3..len];
    let slice_d = &s[3..];

    //to return slice of full index it's as follows.
    let slice_e = &s[0..len]; // specific syntax
    let slice_f = &s[..]; // no specified index it will return full string as a slice.
    println!("slice_a: {slice_a}, slice_b: {slice_b},slice_c: {slice_c}, slice_d: {slice_d}, slice_e: {slice_e}, slice_f: {slice_f},")
}

// `first_word` also works on references to `String`s, which are equivalent
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    // turn string into bytes, iterate and enumerate over this
    for (i, &item) in bytes.iter().enumerate() {
        // item is the number for ascii code of the item.
        println!("item: {item}");

        if item == b' ' {
            // return string slice from index 0 to current index of loop where the whitespace is found.
            return &s[..i];
        }
    }
    // if there is no whitespace then full string slice will be returned
    &s[..]
}

// same logic just return from index to end rather than start to index
fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i+1..];
        }
    }
    &s[..]
}

//This slice has the type &[i32]. It works the same way as string slices do, 
// by storing a reference to the first element and a length. 
// You’ll use this kind of slice for all sorts of other collections
fn other_slice() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}