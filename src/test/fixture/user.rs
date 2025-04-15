use chrono::DateTime;

use crate::models::user_model::User;

pub fn user_fixture(id: usize) -> User {
    User {
        id: id as i32,
        email: String::from("fixture@mail.com"),
        username: String::from("test_user"),
        password: String::from("password"),
        created_at: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
            .unwrap()
            .naive_utc(),
    }
}

pub fn users_fixture(num: usize) -> Vec<User> {
    let mut users = vec![];
    for i in 1..num + 1 {
        let mut user = user_fixture(i);
        // Add unique username for each user
        user.username = format!("test_user_{}", i);
        users.push(user);
    }
    users
}
