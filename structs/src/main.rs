struct AlwaysEqual;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let build_user = build_user(
        String::from("build123"),
        String::from("person123"));
    let build_user_shorthand = build_user_shorthand(
        String::from("build456"), 
        String::from("person456"));

    // Struct update 
    // Causes MOVE of user1 string fields
    /*
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("Another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    */
    // Struct update shorthand
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    // unit-like structs without fields
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

