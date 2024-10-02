use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(pub i64);

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    id: UserId,
    name: String,
    password: String,
}
