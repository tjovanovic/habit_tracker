use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserId(u64);

#[derive(Deserialize, Serialize)]
pub struct User {
    id: UserId,
    name: String,
    password: String,
}
