fn main() {
    let username = String::from("KK0NA");
    let email = String::from("kkona@sub.now");
    let username2 = String::from("Reinad");
    let email2 = String::from("Rei@sub.now");

    let created_struct = create_user_without_update(&username, &email);
    let second_struct: User = create_user_with_update(&username2, &email2);

    println!("User's email is {}", created_struct.email);
    println!("Second user's sign in count is {}", second_struct.sign_in_count);

    tuple_structs();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}



fn tuple_structs(){
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,15,0);

    let Point(x, y, z) = origin;
    let Color(r, g, b) = black;

    println!("Origin's y is {}", y);
}

fn create_user_without_update(user: &str, email: &str) -> User {
    User {
        email: email.to_string(),
        username: user.to_string(),
        active: true,
        sign_in_count: 1,
    }
}

fn create_user_with_update(user: &str, email: &str) -> User {
    let struct_base = User {
        email: String::from("kkona@sub.now"),
        username: String::from("KK0NA"),
        active: true,
        sign_in_count: 12,
    };
    
    User {
        email: email.to_string(),
        username: user.to_string(),
        ..struct_base
    }
}
