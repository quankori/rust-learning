use std::collections::HashMap;
pub fn normal() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("{:?}", user1.active);
    println!("{:?}", user1.username);
    println!("{:?}", user1.sign_in_count);
    println!("{:?}", user1.email);
}

pub fn hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
