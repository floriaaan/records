use chrono::DateTime;

use crate::models::user_model::User;

pub fn user_fixture(id: usize) -> User {
    User {
        id: id as i32,
        email: String::from("taro"),
        password: String::from("password"),
        created_at: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
            .unwrap()
            .naive_utc(),
    }
}

pub fn users_fixture(num: usize) -> Vec<User> {
    let mut users = vec![];
    for i in 1..num + 1 {
        users.push(user_fixture(i));
    }
    users
}
