use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn main() {
    // open_file_handle_result_with_match();
    // open_file_handle_result_cleanly();
    // using_unwrap();
    // using_expect();
    let result = read_username_from_file();
    println!("{result:?}");
    let result_of_shortcut = error_propagation_shortcut();
    println!("{result_of_shortcut:?}");
    let even_shorter_cut = even_shorter_cut();
    println!("{even_shorter_cut:?}");
    let std = shortest();
    println!("{std:?}");
    let option_question_mark = last_char_of_first_line("");
    println!("{option_question_mark:?}");
}

fn open_file_handle_result_with_match() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}
//unwrap or else used to get the file or handle error.
fn open_file_handle_result_cleanly() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

fn using_unwrap() {
    let greeting_file = File::open("nonexistentfile.txt").unwrap();
}

fn using_expect() {
    let greeting_file = File::open("nonexistentfile.txt")
        .expect("nonexistentfile.txt should be included in this project.");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
// ? will return either the ok or the error variant. Very nice syntax.
fn error_propagation_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn even_shorter_cut() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
//std library approach
fn shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
//? with option
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}