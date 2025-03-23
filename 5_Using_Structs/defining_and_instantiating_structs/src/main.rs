
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
// tuple structs

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// fieldless struct 

struct AlwaysEqual

fn main() {
    println!("Hello, world!");
    let user1 = User {
        active: true,
        username: String::from("somerandomusername"),
        email: String::from("somerandom@email.com"),
        sign_in_count: 1
    };
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("randomemail@gmail.com"),
    //     sign_in_count: user1.sign_in_count
    // };
    // uses the rest of the fields from user1 after email is set.
    let user3 = User {
        email: String::from("specific@email.com"),
        ..user1
    };
    let user4 = build_user(String::from("username"), String::from("email@provider.com"));
    
    // instantiating a tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // destructure tuple struct
    let Color(a, b, c) = black;
    let Point(x, y, z) = origin;
    println!("color: {a}, {b}, {c}");
    println!("point: {x}, {y}, {z}");

    // fieldless struct
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}