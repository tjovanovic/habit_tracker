use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserId(pub u64);

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    id: UserId,
    name: String,
    password: String,
}
