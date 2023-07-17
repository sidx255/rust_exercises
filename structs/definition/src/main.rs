struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        username: String::from("user1"),
        email: String::from("user@mail.com"),
        sign_in_count: 1,
        active: true,
    };

    // access value
    let name = user1.username;

    // change value
    user1.username = String::from("user0");

    // create using function
    let user2 = build_user(String::from("user2@mail.com"), String::from("user2"));

    // create using existing instance
    let user3 = User {
        username: String::from("user3"),
        email: String::from("user3@mail.com"),
        ..user2
    };

    // tuple struct
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    // unit-like struct
    struct UnitLike();

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


